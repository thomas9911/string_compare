#![feature(test)]

extern crate test;

use string_compare::*;
use test::Bencher;

const SMALL_A: &'static str = "hallohallohallohallohallo";
const SMALL_B: &'static str = "hallohallohallohallohallb";

const MEDIUM_A: &'static str = "hallohallohallohallohallohallohallohallohallohallohallohallohallohallo";
const MEDIUM_B: &'static str = "hallohallohallohallohallohallohallohallohallohallohallohallohallohallb";

#[bench]
fn small_compare_rust(b: &mut Bencher) {
    b.iter(|| {
        let a = test::black_box(SMALL_A);
        let b = test::black_box(SMALL_B);
        compare_rust(a, b)
    });
}

#[bench]
fn small_compare_at_home(b: &mut Bencher) {
    b.iter(|| {
        let a = test::black_box(SMALL_A);
        let b = test::black_box(SMALL_B);
        compare_at_home(a, b)
    });
}

#[bench]
fn small_compare_asm(b: &mut Bencher) {
    b.iter(|| {
        let a = test::black_box(SMALL_A);
        let b = test::black_box(SMALL_B);
        compare_asm(a, b)
    });
}


#[bench]
fn medium_compare_rust(b: &mut Bencher) {
    b.iter(|| {
        let a = test::black_box(MEDIUM_A);
        let b = test::black_box(MEDIUM_B);
        compare_rust(a, b)
    });
}

#[bench]
fn medium_compare_at_home(b: &mut Bencher) {
    b.iter(|| {
        let a = test::black_box(MEDIUM_A);
        let b = test::black_box(MEDIUM_B);
        compare_at_home(a, b)
    });
}

#[bench]
fn medium_compare_asm(b: &mut Bencher) {
    b.iter(|| {
        let a = test::black_box(MEDIUM_A);
        let b = test::black_box(MEDIUM_B);
        compare_asm(a, b)
    });
}
