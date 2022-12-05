use std::collections::BTreeMap;

#[derive(Debug)]
struct Storage<'a> {
    rows: Vec<&'a str>,
    data: BTreeMap<&'a str, Vec<&'a str>>,
}

impl<'a> std::ops::Deref for Storage<'a> {
    type Target = BTreeMap<&'a str, Vec<&'a str>>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<'a> std::ops::DerefMut for Storage<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<'a> Storage<'a> {
    pub fn new(rows: Vec<&'a str>) -> Storage<'a> {
        let mut data = BTreeMap::new();

        for i in rows.iter() {
            data.insert(*i, Vec::new());
        }

        Storage { data, rows }
    }

    pub fn parse_instruction(
        mut input: &'a str,
    ) -> Result<(usize, &'a str, &'a str), Box<dyn std::error::Error>> {
        input = if let Some((_, rest)) = input.split_once("move ") {
            rest
        } else {
            return Err("invalid line".into());
        };

        let result = if let Some((amount, rest)) = input.split_once(" from ") {
            (amount, rest)
        } else {
            return Err("invalid line".into());
        };

        let amount: usize = result.0.parse()?;
        input = result.1;

        let result = if let Some((target, rest)) = input.split_once(" to ") {
            (target, rest)
        } else {
            return Err("invalid line".into());
        };

        let from = result.0;
        let to = result.1;

        Ok((amount, from, to))
    }

    pub fn apply_line(
        &mut self,
        input: &'a str,
        multimover: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (amount, from, to) = Self::parse_instruction(input)?;

        let mut move_part = if let Some(column_data) = self.data.get_mut(from) {
            column_data.split_off(column_data.len().saturating_sub(amount))
        } else {
            return Err("invalid row".into());
        };

        if !multimover {
            // moved one at a time
            move_part.reverse();
        }

        if let Some(column_data) = self.data.get_mut(to) {
            column_data.extend(move_part);
        } else {
            return Err("invalid row".into());
        };

        Ok(())
    }

    pub fn parse(input: &'a str) -> Result<Storage<'a>, Box<dyn std::error::Error>> {
        let mut line_iterator = input.lines().rev();
        let last_line = line_iterator.next().ok_or_else(|| "invalid last line")?;
        let row_indexes: Vec<&str> = last_line.split_whitespace().collect();
        let mut storage = Storage::new(row_indexes);

        for line in line_iterator {
            for (container, item) in Self::parse_line(line).into_iter().zip(storage.rows.iter()) {
                if let Some(container) = container {
                    storage
                        .data
                        .entry(item)
                        .or_insert(Vec::new())
                        .push(container);
                }
            }
        }

        Ok(storage)
    }

    fn parse_line(input: &'a str) -> Vec<Option<&'a str>> {
        let mut line = Vec::new();
        let mut item_start = None;
        let mut whitespaces = 0;
        for (index, ch) in input.chars().enumerate() {
            if ch == '[' {
                whitespaces = 0;
                continue;
            }

            if ch == ']' {
                if let Some(start) = item_start {
                    line.push(Some(&input[start..index]));
                    item_start = None;
                }
                continue;
            }

            if ch == ' ' {
                whitespaces += 1;

                if (whitespaces % 4) == 2 {
                    line.push(None);
                }
                continue;
            }

            if item_start.is_none() {
                item_start = Some(index);
            }
        }

        line
    }

    pub fn list_upper_containers(&self) -> String {
        let mut output = String::with_capacity(self.rows.len());
        for row in self.rows.iter() {
            if let Some(data) = self.data.get(row) {
                output.push_str(data.last().unwrap_or(&""))
            }
        }

        output
    }
}

fn run(input_path: &str, multimover: bool) -> String {
    let input = crate::load_to_string(input_path).unwrap();

    let (begin_state_text, instructions_text) = if let Some(result) = input.split_once("\n\n") {
        result
    } else {
        return String::new();
    };

    let mut storage = Storage::parse(begin_state_text).unwrap();

    for line in instructions_text.lines() {
        storage.apply_line(line, multimover).unwrap();
    }

    storage.list_upper_containers()
}

pub fn day05a() -> String {
    run("data/day05/day05a.txt", false)
}

pub fn day05b() -> String {
    run("data/day05/day05b.txt", true)
}
