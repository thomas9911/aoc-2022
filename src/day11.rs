use std::collections::{BTreeMap, BinaryHeap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

type InterestLevelInteger = u32;
type Monkeys = BTreeMap<usize, Monkey>;

#[derive(Debug)]
pub enum Operation {
    Pow(u32),
    Add(InterestLevelInteger),
    Mul(InterestLevelInteger),
}

impl Operation {
    pub fn apply(&self, item: InterestLevelInteger) -> InterestLevelInteger {
        use Operation::*;

        match self {
            Pow(y) => item.pow(*y),
            Add(y) => item.saturating_add(*y),
            Mul(y) => item.saturating_mul(*y),
        }
    }

    pub fn apply_modulo(
        &self,
        item: InterestLevelInteger,
        modulo: InterestLevelInteger,
    ) -> InterestLevelInteger {
        use Operation::*;

        match self {
            Pow(y) => {
                let item = item as u128;
                let result = (item % (modulo as u128)).pow(*y);
                (result % (modulo as u128)) as InterestLevelInteger
            }
            Add(y) => (item.saturating_add(*y)) % modulo,
            Mul(y) => (item.saturating_mul(*y)) % modulo,
        }
    }
}

#[derive(Debug)]
pub struct Test {
    test: InterestLevelInteger,
    if_true: usize,
    if_false: usize,
}

impl Test {
    pub fn next_monkey(&self, item: InterestLevelInteger) -> usize {
        self.next_monkey_with_result(item).1
    }

    pub fn next_monkey_with_result(&self, item: InterestLevelInteger) -> (bool, usize) {
        if item % self.test == 0 {
            (true, self.if_true)
        } else {
            (false, self.if_false)
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Monkey {
    name: String,
    starting_items: VecDeque<InterestLevelInteger>,
    operation: Operation,
    test: Test,
}

pub fn parse_file(file: File) -> Monkeys {
    let reader = BufReader::new(file);
    let mut monkeys: Monkeys = BTreeMap::new();

    let mut starting_items = VecDeque::new();
    let mut name = String::new();
    let mut operation = None;
    let mut test = 1;
    let mut test_if_true = 0;
    let mut test_if_false = 0;
    let mut index = 0;

    for line in reader.lines().chain(std::iter::once(Ok("".to_string()))) {
        let line = line.expect("invalid line");
        if line == "" {
            // new monkey
            monkeys.insert(
                index,
                Monkey {
                    name,
                    starting_items: starting_items.drain(..).collect(),
                    operation: operation.expect("operation not found"),
                    test: Test {
                        test,
                        if_true: test_if_true,
                        if_false: test_if_false,
                    },
                },
            );
            name = String::new();
            operation = None;
            index += 1;
            continue;
        }

        if line.starts_with("Monkey ") {
            name = line.trim_end_matches(':').to_string();
            continue;
        }

        let trimmed_line = line.trim();
        if let Some((_, csv)) = trimmed_line.split_once("Starting items: ") {
            starting_items = csv.split(", ").map(|x| x.parse().unwrap()).collect();

            continue;
        }

        if trimmed_line == "Operation: new = old * old" {
            operation = Some(Operation::Pow(2));
            continue;
        }

        if let Some((_, function)) = trimmed_line.split_once("Operation: new = old ") {
            let found_operation = match function.chars().next() {
                Some('*') => {
                    Operation::Mul(function.split_at(2).1.parse().expect("invalid operation"))
                }
                Some('+') => {
                    Operation::Add(function.split_at(2).1.parse().expect("invalid operation"))
                }
                _ => panic!("invalid operation"),
            };

            operation = Some(found_operation);

            continue;
        }

        if let Some((_, div_test)) = trimmed_line.split_once("Test: divisible by ") {
            test = div_test.parse().expect("invalid test");
            continue;
        }

        if let Some((_, true_test)) = trimmed_line.split_once("If true: throw to monkey ") {
            test_if_true = true_test.parse().expect("invalid true test");
            continue;
        }

        if let Some((_, false_test)) = trimmed_line.split_once("If false: throw to monkey ") {
            test_if_false = false_test.parse().expect("invalid false test");
            continue;
        }
    }

    monkeys
}

fn play_round(monkeys: &mut Monkeys, counter: &mut BTreeMap<usize, usize>) {
    let mut index = 0;

    while index < monkeys.len() {
        // temporary remove playing monkey from the monkeys collection, because it can't throw to itself
        let mut playing_monkey = monkeys.remove(&index).expect("monkey not found");

        while let Some(item) = playing_monkey.starting_items.pop_front() {
            let mut item = item.clone();
            item = playing_monkey.operation.apply(item);
            item /= 3; // the operation that is never used :O (/=)

            let entry = counter.entry(index).or_insert(0);
            *entry += 1;

            let next_monkey = playing_monkey.test.next_monkey(item);
            let to_monkey = monkeys.get_mut(&next_monkey).expect("to monkey not found");
            to_monkey.starting_items.push_back(item);
        }

        monkeys.insert(index, playing_monkey);
        index += 1;
    }
}

fn play_round_modulo(
    monkeys: &mut Monkeys,
    counter: &mut BTreeMap<usize, usize>,
    modulo: InterestLevelInteger,
) {
    let mut index = 0;

    while index < monkeys.len() {
        // temporary remove playing monkey from the monkeys collection, because it can't throw to itself
        let mut playing_monkey = monkeys.remove(&index).expect("monkey not found");

        while let Some(item) = playing_monkey.starting_items.pop_front() {
            let mut item = item.clone();
            item = playing_monkey.operation.apply_modulo(item, modulo);

            let entry = counter.entry(index).or_insert(0);
            *entry += 1;

            item = item % modulo;

            let next_monkey = playing_monkey.test.next_monkey(item);

            let to_monkey = monkeys.get_mut(&next_monkey).expect("to monkey not found");
            to_monkey.starting_items.push_back(item);
        }

        monkeys.insert(index, playing_monkey);
        index += 1;
    }
}

pub fn monkey_bussiness(counter: &BTreeMap<usize, usize>) -> usize {
    let mut heap = BinaryHeap::new();

    for value in counter.values() {
        heap.push(*value);
    }

    let largest = heap.pop().expect("empty counter");
    let next_largest = heap.pop().expect("empty counter");

    largest * next_largest
}

pub fn day11a() -> usize {
    let file = File::open("data/day11/day11a.txt").expect("data file not found");
    let mut monkeys = parse_file(file);
    let mut counter: BTreeMap<usize, usize> = monkeys.keys().map(|key| (*key, 0)).collect();

    for _ in 0..20 {
        play_round(&mut monkeys, &mut counter);
    }

    monkey_bussiness(&counter)
}

pub fn day11b() -> usize {
    let file = File::open("data/day11/day11b.txt").expect("data file not found");
    let mut monkeys = parse_file(file);
    let mut counter: BTreeMap<usize, usize> = monkeys.keys().map(|key| (*key, 0)).collect();

    let modulo = monkeys.values().map(|x| x.test.test).product();

    for _ in 0..10000 {
        play_round_modulo(&mut monkeys, &mut counter, modulo);
    }

    monkey_bussiness(&counter)
}
