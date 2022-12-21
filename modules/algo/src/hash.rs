use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

enum HashItem<K: Hash + PartialEq + Eq, V> {
    Empty,
    Delete,
    Item(K, V),
}

use HashItem::*;

struct HashTable<K: Hash + PartialEq + Eq, V> {
    table: Vec<HashItem<K, V>>,
    limit: usize,
    size: usize,
}

impl<K: Hash + PartialEq + Eq, V> HashTable<K, V> {
    fn new(n: usize) -> HashTable<K, V> {
        let mut ht = Vec::new();
        for _ in 0..n {
            ht.push(Empty);
        }
        HashTable { table: ht, limit: n, size: 0 }
    }

    fn get_index(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as usize % self.limit
    }

    fn get_key_index(&self, key: &K) -> Option<usize> {
        let mut i = self.get_index(key);
        let mut cnt = 0;
        while cnt < self.limit {
            match self.table[i] {
                Empty => return None,
                Item(ref k, _) if key == k => return Some(i),
                _ => (),
            }
            cnt += 1;
            i += 1;
            if i == self.limit {
                i = 0;
            }
        }
        None
    }

    fn contains_key(&self, key: &K) -> bool {
        self.get_key_index(key).is_some()
    }

    fn get(&self, key: &K) -> Option<&V> {
        match self.get_key_index(key) {
            None => None,
            Some(i) => match &self.table[i] {
                &Item(_, ref v) => Some(v),
                _ => None,
            },
        }
    }

    fn insert(&mut self, key: K, value: V) -> bool {
        let mut i = self.get_index(&key);
        let mut cnt = 0;
        while cnt < self.limit {
            let p = &mut self.table[i];
            match p {
                &mut Empty | &mut Delete => {
                    *p = Item(key, value);
                    self.size += 1;
                    return true;
                }
                &mut Item(ref k, ref mut v) if *k == key => {
                    *v = value;
                    return true;
                }
                _ => (),
            }
            cnt += 1;
            i += 1;
            if i == self.limit {
                i = 0;
            }
        }
        false
    }

    fn remove(&mut self, key: &K) -> bool {
        match self.get_key_index(key) {
            None => false,
            Some(i) => {
                self.table[i] = Delete;
                self.size -= 1;
                true
            }
        }
    }

    fn len(&self) -> usize {
        self.size
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.table.len()
    }

    fn clear(&mut self) {
        for i in 0..self.table.len() {
            self.table[i] = Empty;
        }
        self.size = 0;
    }
}

#[derive(Hash, PartialEq, Eq)]
struct Foo {
    num: i32,
}

pub fn execute() {
    println!("hash");

    let mut ht: HashTable<Foo, i32> = HashTable::new(11);
    println!("{}", ht.len());
    println!("{}", ht.is_empty());
    println!("{}", ht.is_full());

    for x in 1..12 {
        ht.insert(Foo { num: x }, x * 10);
    }
    println!("{}", ht.len());
    println!("{}", ht.is_empty());
    println!("{}", ht.is_full());

    for x in 0..13 {
        let key = Foo { num: x };
        println!("{}", ht.contains_key(&key));
        println!("{:?}", ht.get(&key));
        println!("{}", ht.remove(&key));
        println!("{}", ht.contains_key(&key));
    }
    println!("{}", ht.len());
    println!("{}", ht.is_empty());
    println!("{}", ht.is_full());

    for x in 0..11 {
        ht.insert(Foo { num: x }, x * 10);
    }
    println!("{}", ht.len());
    println!("{}", ht.is_empty());
    println!("{}", ht.is_full());

    ht.clear();
    println!("{}", ht.len());
    println!("{}", ht.is_empty());
    println!("{}", ht.is_full());
}
