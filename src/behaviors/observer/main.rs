trait Subject<T: Clone> {
    fn notify_observers(&self, _: &T);
    fn register_observer(&mut self, _: Box<dyn Observer<T>>) -> usize;
    fn unregister_observer(&mut self, _: usize);
}

trait Observer<T: Clone> {
    fn on_notify(&self, _: &T);
}

#[derive(Debug, Clone)]
struct EventObject(usize);

struct SubjectX {
    observers: Vec<(bool, Box<dyn Observer<EventObject>>)>,
}
impl SubjectX {
    fn new() -> Self {
        Self {
            observers: Vec::new(),
        }
    }
}

impl Subject<EventObject> for SubjectX {
    fn notify_observers(&self, e: &EventObject) {
        for observer in self.observers.iter() {
            if observer.0 {
                observer.1.on_notify(e);
            }
        }
    }

    fn register_observer(&mut self, o: Box<dyn Observer<EventObject>>) -> usize {
        self.observers.push((true, o));
        self.observers.len() - 1
    }

    fn unregister_observer(&mut self, i: usize) {
        self.observers[i].0 = false
    }
}

struct ObserverX(usize);
impl Observer<EventObject> for ObserverX {
    fn on_notify(&self, e: &EventObject) {
        println!("ObserverX {} Get {:?}", self.0, e);
    }
}

fn execute() {
    let mut subject = SubjectX::new();
    subject.register_observer(Box::new(ObserverX(1)));
    subject.register_observer(Box::new(ObserverX(2)));
    subject.register_observer(Box::new(ObserverX(3)));

    subject.notify_observers(&EventObject(100));
    subject.notify_observers(&EventObject(20));
}

pub fn output() {
    execute();
}

#[allow(dead_code)]
fn main() {
    output();
}

#[cfg(test)]
mod main_test;