use rand;
use rand::prelude::*;
use std::fmt;
use std::process;
use std::{thread, time};

trait State: fmt::Display {
    fn do_clock(&self, hour: u32) -> Box<dyn State>;
    fn do_use(&self, context: &dyn Context);
    fn do_alarm(&self, context: &dyn Context);
    fn do_phone(&self, context: &dyn Context);
    fn value(&self) -> String;
}

impl PartialEq<dyn State> for dyn State {
    fn eq(&self, other: &dyn State) -> bool {
        self.value() == other.value()
    }
}

trait Context {
    fn set_clock(&mut self, hour: u32);
    fn change_state(&mut self, state: Box<dyn State>);
    fn call_security_center(&self, msg: String);
    fn record_log(&self, msg: String);
}

#[derive(PartialEq)]
struct DayState {}

impl DayState {
    fn new() -> DayState {
        DayState {}
    }
}

impl State for DayState {
    fn do_clock(&self, hour: u32) -> Box<dyn State> {
        if !(9..17).contains(&hour) {
            Box::new(NightState::new())
        } else {
            Box::new(DayState::new())
        }
    }

    fn do_use(&self, context: &dyn Context) {
        context.record_log("金庫使用(昼間)".to_string());
    }

    fn do_alarm(&self, context: &dyn Context) {
        context.call_security_center("非常ベル(昼間)".to_string());
    }

    fn do_phone(&self, context: &dyn Context) {
        context.call_security_center("通常の通話(昼間)".to_string());
    }

    fn value(&self) -> String {
        "昼間".to_string()
    }
}

impl fmt::Display for DayState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[昼間]")
    }
}

#[derive(PartialEq)]
struct NightState {}

impl NightState {
    fn new() -> NightState {
        NightState {}
    }
}

impl State for NightState {
    fn do_clock(&self, hour: u32) -> Box<dyn State> {
        if (9..17).contains(&hour) {
            Box::new(DayState::new())
        } else {
            Box::new(NightState::new())
        }
    }

    fn do_use(&self, context: &dyn Context) {
        context.call_security_center("非常:夜間の金庫使用!".to_string());
    }

    fn do_alarm(&self, context: &dyn Context) {
        context.call_security_center("非常ベル(夜間)".to_string());
    }

    fn do_phone(&self, context: &dyn Context) {
        context.record_log("夜間の通話録音".to_string());
    }

    fn value(&self) -> String {
        "夜間".to_string()
    }
}

impl fmt::Display for NightState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[夜間]")
    }
}

struct SafeFrame {
    title: String,
    state: Box<dyn State>,
}

impl SafeFrame {
    fn new(title: String, state: Box<dyn State>) -> SafeFrame {
        SafeFrame { title, state }
    }

    fn click_use(&self) {
        self.state.do_use(self);
    }

    fn click_alarm(&self) {
        self.state.do_alarm(self);
    }

    fn click_phone(&self) {
        self.state.do_phone(self);
    }

    fn click_exit(&self) {
        process::exit(0);
    }
}

impl Context for SafeFrame {
    fn set_clock(&mut self, hour: u32) {
        println!("現在時刻は{0: >02}:00", hour);

        let state = self.state.do_clock(hour);
        #[allow(clippy::op_ref)]
        if &self.state != &state {
            self.change_state(state);
        }
    }

    fn change_state(&mut self, state: Box<dyn State>) {
        println!("{}から{}へ状態が変化しました。", self.state, state);
        self.state = state;
    }

    fn call_security_center(&self, msg: String) {
        println!("call! {}", msg);
    }

    fn record_log(&self, msg: String) {
        println!("record ... {}", msg);
    }
}

pub fn execute() {
    println!("state");

    let mut frame = SafeFrame::new("State Sample".to_string(), Box::new(NightState::new()));
    let mut rng = thread_rng();

    println!("------------");
    println!("{}", frame.title);
    println!("------------\n");

    loop {
        for hour in 0..24 {
            frame.set_clock(hour);

            match rng.gen_range(0..3) {
                0 => frame.click_use(),
                1 => frame.click_alarm(),
                2 => frame.click_phone(),
                _ => frame.click_exit(),
            }

            thread::sleep(time::Duration::from_millis(1000));
        }
    }
}
