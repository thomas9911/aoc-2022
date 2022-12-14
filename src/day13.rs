use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fmt::Display;
use std::str::Chars;

struct Multiline<'a> {
    text: &'a str,
}

impl<'a> Iterator for Multiline<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((line, text)) = self.text.split_once("\r\n\r\n") {
            self.text = text;
            return Some(line);
        }
        if let Some((line, text)) = self.text.split_once("\n\n") {
            self.text = text;
            return Some(line);
        }

        if !self.text.is_empty() {
            // strip \r\n
            let mut text = self.text;
            if text.ends_with('\n') {
                text = &text[..text.len() - 1];
                if text.ends_with('\r') {
                    text = &text[..text.len() - 1];
                }
            }

            self.text = "";
            return Some(text);
        }

        None
    }
}

type MyResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Item {
    Value(usize),
    List(VecDeque<Item>),
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Item::Value(x) => write!(f, "{}", x),
            Item::List(x) => {
                write!(f, "[")?;
                for item in x {
                    write!(f, "{},", item)?;
                }
                write!(f, "]")
            }
        }
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Item::*;
        match (self, other) {
            (Value(x), Value(y)) => x.partial_cmp(y),
            (List(left), List(right)) => {
                for (x, y) in left.iter().zip(right.iter()) {
                    match x.partial_cmp(y) {
                        Some(Ordering::Less) => return Some(Ordering::Less),
                        Some(Ordering::Greater) => return Some(Ordering::Greater),
                        _ => continue,
                    }
                }
                left.len().partial_cmp(&right.len())
            }
            (x, Value(y)) => {
                let list_y = Item::List(VecDeque::from_iter([Value(*y)]));
                x.partial_cmp(&list_y)
            }
            (Value(x), y) => {
                let list_x = Item::List(VecDeque::from_iter([Value(*x)]));
                list_x.partial_cmp(y)
            }
        }
    }
}

fn parse_item(item: &str) -> MyResult<Item> {
    parse_item_inner(&mut item.chars())
}

fn parse_item_inner(item: &mut Chars) -> MyResult<Item> {
    let mut list = VecDeque::new();
    let mut val = String::new();

    while let Some(ch) = item.next() {
        match ch {
            '[' => {
                list.push_back(parse_item_inner(item)?);
            }
            '0'..='9' => val.push(ch),
            ']' => {
                if !val.is_empty() {
                    list.push_back(Item::Value(val.parse()?));
                    val.clear();
                }

                return Ok(Item::List(list));
            }
            ',' => {
                if !val.is_empty() {
                    list.push_back(Item::Value(val.parse()?));
                    val.clear();
                }
            }
            _ => return Err("invalid char".into()),
        }
    }

    Ok(Item::List(list))
}

fn parse_pair(input: &str) -> MyResult<(Item, Item)> {
    let pair: Vec<_> = input.lines().collect();
    if pair.len() != 2 {
        return Err("invalid pair".into());
    };

    let left = parse_item(pair[0])?;
    let right = parse_item(pair[1])?;

    Ok((left, right))
}

pub fn day13a() -> usize {
    let text = std::fs::read_to_string("data/day13/day13a.txt").unwrap();
    let mut sum_of_less = 0;
    for (index, pair) in (Multiline { text: &text }).enumerate() {
        let (left, right) = parse_pair(pair).unwrap();
        let ordering = left.partial_cmp(&right).unwrap();
        if ordering == Ordering::Less {
            // zero based index add one
            sum_of_less += index + 1;
        }
    }

    sum_of_less
}

pub fn day13b() -> usize {
    let text = std::fs::read_to_string("data/day13/day13b.txt").unwrap();
    let first_divider = Item::List([Item::List([Item::Value(2)].into())].into());
    let second_divider = Item::List([Item::List([Item::Value(6)].into())].into());

    let lines: MyResult<Vec<_>> = text
        .lines()
        .filter(|text| text != &"")
        .map(parse_item)
        .collect();
    let mut lines = lines.unwrap();
    lines.push(first_divider.clone());
    lines.push(second_divider.clone());

    lines.sort();

    let first_div_zero_index = lines.iter().position(|x| x == &first_divider).unwrap();
    let second_div_zero_index = lines.iter().position(|x| x == &second_divider).unwrap();

    (first_div_zero_index + 1) * (second_div_zero_index + 1)
}
