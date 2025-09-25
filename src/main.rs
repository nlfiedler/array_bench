//
// Copyright (c) 2025 Nathan Fiedler
//
use extarray::ExtensibleArray;
use optarray::OptimalArray as BrodnikArray;
use segment_array::SegmentArray;
use std::time::{Duration, Instant};
use tzarrays::general::OptimalArray as GeneralArray;
use tzarrays::simple::OptimalArray as SimpleArray;

struct Times {
    create: Duration,
    ordered: Duration,
    popall: Duration,
}

/// Drop the low and high values, return average of those that remain.
fn compute_average(mut times: Vec<Duration>) -> u64 {
    times.sort();
    let total: u64 = times[1..times.len() - 2]
        .iter()
        .fold(0, |acc, x| acc + x.as_millis() as u64);
    total / (times.len() as u64 - 2)
}

/// Show the average for the collected running times.
fn display_average_times(times: Vec<Times>) {
    let create: Vec<Duration> = times.iter().map(|t| t.create).collect();
    let create = compute_average(create);
    let ordered: Vec<Duration> = times.iter().map(|t| t.ordered).collect();
    let ordered = compute_average(ordered);
    let popall: Vec<Duration> = times.iter().map(|t| t.popall).collect();
    let popall = compute_average(popall);
    println!("create: {create}, ordered: {ordered}, pop-all: {popall}",);
}

fn benchmark_segarray(size: usize) -> Times {
    let mut coll: SegmentArray<usize> = SegmentArray::new();
    let start = Instant::now();
    for value in 0..size {
        coll.push(value);
    }
    let create = start.elapsed();

    // test sequenced access for entire collection
    let start = Instant::now();
    for (index, value) in coll.iter().enumerate() {
        assert_eq!(*value, index);
    }
    let ordered = start.elapsed();

    // test popping all elements from the array
    let start = Instant::now();
    while !coll.is_empty() {
        coll.pop();
    }
    let popall = start.elapsed();
    Times {
        create,
        ordered,
        popall,
    }
}

fn benchmark_optarray(size: usize) -> Times {
    let mut coll: BrodnikArray<usize> = BrodnikArray::new();
    let start = Instant::now();
    for value in 0..size {
        coll.push(value);
    }
    let create = start.elapsed();

    // test sequenced access for entire collection
    let start = Instant::now();
    for (index, value) in coll.iter().enumerate() {
        assert_eq!(*value, index);
    }
    let ordered = start.elapsed();

    // test popping all elements from the array
    let start = Instant::now();
    while !coll.is_empty() {
        coll.pop();
    }
    let popall = start.elapsed();
    Times {
        create,
        ordered,
        popall,
    }
}

fn benchmark_extarray(size: usize) -> Times {
    let mut coll: ExtensibleArray<usize> = ExtensibleArray::new();
    let start = Instant::now();
    for value in 0..size {
        coll.push(value);
    }
    let create = start.elapsed();

    // test sequenced access for entire collection
    let start = Instant::now();
    for (index, value) in coll.iter().enumerate() {
        assert_eq!(*value, index);
    }
    let ordered = start.elapsed();

    // test popping all elements from the array
    let start = Instant::now();
    while !coll.is_empty() {
        coll.pop();
    }
    let popall = start.elapsed();
    Times {
        create,
        ordered,
        popall,
    }
}

fn benchmark_general_tarjan(coll: &mut GeneralArray<usize>, size: usize) -> Times {
    let start = Instant::now();
    for value in 0..size {
        coll.push(value);
    }
    let create = start.elapsed();

    // test sequenced access for entire collection
    let start = Instant::now();
    for (index, value) in coll.iter().enumerate() {
        assert_eq!(*value, index);
    }
    let ordered = start.elapsed();

    // test popping all elements from the array
    let start = Instant::now();
    while !coll.is_empty() {
        coll.pop();
    }
    let popall = start.elapsed();
    Times {
        create,
        ordered,
        popall,
    }
}

fn benchmark_simple_tarjan(coll: &mut SimpleArray<usize>, size: usize) -> Times {
    let start = Instant::now();
    for value in 0..size {
        coll.push(value);
    }
    let create = start.elapsed();

    // test sequenced access for entire collection
    let start = Instant::now();
    for (index, value) in coll.iter().enumerate() {
        assert_eq!(*value, index);
    }
    let ordered = start.elapsed();

    // test popping all elements from the array
    let start = Instant::now();
    while !coll.is_empty() {
        coll.pop();
    }
    let popall = start.elapsed();
    Times {
        create,
        ordered,
        popall,
    }
}

fn benchmark_vector(size: usize) -> Times {
    let start = Instant::now();
    let mut coll: Vec<usize> = Vec::new();
    for value in 0..size {
        coll.push(value);
    }
    let create = start.elapsed();

    // test sequenced access for entire collection
    let start = Instant::now();
    for (index, value) in coll.iter().enumerate() {
        assert_eq!(*value, index);
    }
    let ordered = start.elapsed();

    // test popping all elements from the vector
    let start = Instant::now();
    while !coll.is_empty() {
        coll.pop();
    }
    let popall = start.elapsed();
    Times {
        create,
        ordered,
        popall,
    }
}

fn main() {
    let size = 100_000_000;

    println!("measuring std::vec::Vec...");
    let mut times: Vec<Times> = vec![];
    for _ in 0..7 {
        times.push(benchmark_vector(size));
    }
    display_average_times(times);

    println!("measuring SegmentArray...");
    let mut times: Vec<Times> = vec![];
    for _ in 0..7 {
        times.push(benchmark_segarray(size));
    }
    display_average_times(times);

    println!("measuring OptimalArray...");
    let mut times: Vec<Times> = vec![];
    for _ in 0..7 {
        times.push(benchmark_optarray(size));
    }
    display_average_times(times);

    println!("measuring ExtensibleArray...");
    let mut times: Vec<Times> = vec![];
    for _ in 0..7 {
        times.push(benchmark_extarray(size));
    }
    display_average_times(times);

    println!("measuring GeneralArray (r=3)...");
    let mut coll: GeneralArray<usize> = GeneralArray::new();
    let mut times: Vec<Times> = vec![];
    for _ in 0..7 {
        times.push(benchmark_general_tarjan(&mut coll, size));
    }
    display_average_times(times);

    println!("creating GeneralArray (r=4)...");
    let mut coll: GeneralArray<usize> = GeneralArray::with_r(4);
    let mut times: Vec<Times> = vec![];
    for _ in 0..7 {
        times.push(benchmark_general_tarjan(&mut coll, size));
    }
    display_average_times(times);

    println!("measuring SimpleArray...");
    let mut coll: SimpleArray<usize> = SimpleArray::new();
    let mut times: Vec<Times> = vec![];
    for _ in 0..7 {
        times.push(benchmark_simple_tarjan(&mut coll, size));
    }
    display_average_times(times);
}
