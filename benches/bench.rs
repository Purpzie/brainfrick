#![feature(test)]
#![allow(unused_must_use, unused_imports)]

extern crate test;
use test::{black_box, Bencher};

use brainfrick::Brainfuck;

#[bench]
fn purpzie_sucks(b: &mut Bencher) {
    b.iter(|| {
        Brainfuck::execute(include_str!("../tests/purpzie_sucks.bf"));
    });
}

const NUMWARP: &'static str = include_str!("../tests/numwarp.bf");

#[bench]
fn array_size_right(b: &mut Bencher) {
    let heck = Brainfuck::parse(include_str!("../tests/array_size_right.bf")).unwrap();
    b.iter(move || {
        heck.run();
    });
}

#[bench]
fn array_size_left(b: &mut Bencher) {
    let heck = Brainfuck::parse(include_str!("../tests/array_size_left.bf")).unwrap();
    b.iter(move || {
        heck.run();
    })
}

#[bench]
fn numwarp_parse(b: &mut Bencher) {
    b.iter(|| {
        Brainfuck::parse(NUMWARP);
    });
}

#[bench]
fn numwarp_execute(b: &mut Bencher) {
    let p = Brainfuck::parse(NUMWARP).unwrap();
    b.iter(move || {
        p.input("1234");
    });
}
