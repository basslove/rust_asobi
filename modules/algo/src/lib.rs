pub mod basic;
pub mod combination;
pub mod delay;
pub mod derangement;
pub mod euclidean;
pub mod factorize;
pub mod fibonacci;
pub mod hamming;
pub mod hash;
pub mod kuku;
pub mod nqueen;
pub mod pascal;
pub mod permutation;
pub mod power;
pub mod prime;
pub mod queue;
pub mod sort;
pub mod stdv;

pub fn summay() {
    basic::execute();
    sort::bubble::execute();
    sort::quick::execute();
    sort::merge::execute();
    sort::heap::execute();
    kuku::execute();
    stdv::execute();
    pascal::execute();
    prime::execute();
    factorize::execute();
    euclidean::execute();
    power::execute();
    combination::execute();
    permutation::execute();
    nqueen::execute();
    fibonacci::execute();
    hamming::execute();
    queue::execute();
    derangement::execute();
    hash::execute();
    delay::execute();
}
