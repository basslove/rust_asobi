// 無視 https://doc.rust-lang.org/cargo/guide/project-layout.html

mod algo;
mod behaviors;
mod creations;
mod structures;

use algo::basic::main as basic;
use algo::sorts::bubble::main as sorts_bubble;
use algo::sorts::heap::main as sorts_heap;
use algo::sorts::quick::main as sorts_quick;
use behaviors::chain_of_responsibility::main as chain_of_responsibility;
use behaviors::command::main as command;
use behaviors::interpreter::main as interpreter;
use behaviors::iterator::main as iterator;
use behaviors::moment::main as moment;
use behaviors::observer::main as observer;
use behaviors::state::main as state;
use behaviors::strategy::main as strategy;
use behaviors::template::main as template;
use behaviors::visitor::main as visitor;
use creations::builder::main as builder;
use creations::factory::main as factory;
use creations::prototype::main as prototype;
use creations::singleton::main as singleton;
use structures::adapter::main as adapter;
use structures::bridge::main as bridge;
use structures::composite::main as composite;
use structures::decorator::main as decorator;
use structures::facade::main as facade;
use structures::flyweight::main as flyweight;
use structures::proxy::main as proxy;

fn main() {
    println!("main start");

    // dp
    adapter::output();
    bridge::output();
    builder::output();
    chain_of_responsibility::output();
    command::output();
    composite::output();
    decorator::output();
    facade::output();
    factory::output();
    flyweight::output();
    interpreter::output();
    iterator::output();
    moment::output();
    observer::output();
    prototype::output();
    proxy::output();
    singleton::output();
    state::output();
    strategy::output();
    template::output();
    visitor::output();

    // algo
    basic::output();
    sorts_bubble::output();
    sorts_quick::output();
    sorts_bubble::output();
    sorts_heap::output();

    println!("main end");
}
