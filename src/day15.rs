use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::RangeInclusive,
};

type Pair = (i32, i32);
type MyResult<T> = Result<T, Box<dyn std::error::Error>>;

fn manhattan_distance(left: &Pair, right: &Pair) -> i32 {
    (left.0 - right.0).abs() + (left.1 - right.1).abs()
}

#[derive(Debug, PartialEq, Clone)]
struct Sensor {
    pos: Pair,
    closest_beacon: Pair,
    /// distance is always positive but the code is easier when it is i32
    distance: i32,
}

impl Sensor {
    pub fn covers(&self, location: &Pair) -> bool {
        manhattan_distance(&self.pos, location) <= self.distance
    }

    pub fn edges_iter(&self) -> SensorEdgeIter {
        SensorEdgeIter {
            mid_point: self.pos.clone(),
            distance: self.distance + 1,
            offset: (0, self.distance + 1),
            direction: (true, false),
            first: true,
        }
    }
}

struct SensorEdgeIter {
    mid_point: Pair,
    distance: i32,
    offset: Pair,
    direction: (bool, bool),
    first: bool,
}

impl Iterator for SensorEdgeIter {
    type Item = Pair;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.first && self.offset == (0, self.distance) {
            return None;
        }

        let (x, y) = self.offset;

        if x == self.distance {
            self.first = false;
            self.direction.0 = false;
        }

        if x == -self.distance {
            self.first = false;
            self.direction.0 = true;
        }

        if y == self.distance {
            self.direction.1 = false;
        }

        if y == -self.distance {
            self.direction.1 = true;
        }

        let x_asdf = if self.direction.0 {
            self.offset.0 += 1;
            self.mid_point.0 + x
        } else {
            self.offset.0 -= 1;
            self.mid_point.0 + x
        };

        let y_asdf = if self.direction.1 {
            self.offset.1 += 1;
            self.mid_point.1 + y
        } else {
            self.offset.1 -= 1;
            self.mid_point.1 + y
        };

        Some((x_asdf, y_asdf))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Sensors {
    sensors: Vec<Sensor>,
    range_x: RangeInclusive<i32>,
    range_y: RangeInclusive<i32>,
}

impl Sensors {
    pub fn from_file(file: File) -> MyResult<Sensors> {
        let reader = BufReader::new(file);
        let mut sensors = Vec::new();
        for line in reader.lines() {
            let line = line?;
            let (_, line) = line.split_once("Sensor at x=").ok_or("invalid line")?;
            let (x, line) = line.split_once(", y=").ok_or("invalid line")?;
            let (y, line) = line
                .split_once(": closest beacon is at x=")
                .ok_or("invalid line")?;
            let pos: Pair = (x.parse()?, y.parse()?);

            let (x, y) = line.split_once(", y=").ok_or("invalid line")?;
            let closest_beacon: Pair = (x.parse()?, y.parse()?);
            let distance = manhattan_distance(&pos, &closest_beacon);

            sensors.push(Sensor {
                pos,
                closest_beacon,
                distance,
            });
        }

        let mut range_x = (i32::MAX, i32::MIN);
        let mut range_y = (i32::MAX, i32::MIN);
        for sensor in sensors.iter() {
            let lowest_x = sensor.pos.0 - sensor.distance;
            let lowest_y = sensor.pos.1 - sensor.distance;
            let highest_x = sensor.pos.0 + sensor.distance;
            let highest_y = sensor.pos.1 + sensor.distance;

            if lowest_x < range_x.0 {
                range_x.0 = lowest_x;
            }
            if lowest_y < range_y.0 {
                range_y.0 = lowest_y;
            }
            if highest_x > range_x.1 {
                range_x.1 = highest_x;
            }
            if highest_y > range_y.1 {
                range_y.1 = highest_y;
            }
        }

        Ok(Sensors {
            sensors,
            range_x: RangeInclusive::new(range_x.0, range_x.1),
            range_y: RangeInclusive::new(range_y.0, range_y.1),
        })
    }

    pub fn print_covered_row(&self, row: i32) {
        for i in self.range_x.clone() {
            let mut covered = false;
            let mut is_beacon = false;
            let mut is_sensor = false;
            for sensor in self.sensors.iter() {
                if sensor.covers(&(i, row)) {
                    covered = true;
                    break;
                }
            }

            for sensor in self.sensors.iter() {
                if &sensor.closest_beacon == &(i, row) {
                    covered = false;
                    is_beacon = true;
                    break;
                }
            }

            for sensor in self.sensors.iter() {
                if &sensor.pos == &(i, row) {
                    covered = false;
                    is_sensor = true;
                    break;
                }
            }

            if covered {
                print!("#");
            } else if is_sensor {
                print!("S");
            } else if is_beacon {
                print!("B");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }

    pub fn count_covered_row(&self, row: i32) -> usize {
        let mut count = 0;
        for i in self.range_x.clone() {
            let mut covered = false;
            for sensor in self.sensors.iter() {
                if sensor.covers(&(i, row)) {
                    covered = true;
                    break;
                }
            }

            for sensor in self.sensors.iter() {
                if &sensor.closest_beacon == &(i, row) {
                    covered = false;
                    break;
                }
            }

            for sensor in self.sensors.iter() {
                if &sensor.pos == &(i, row) {
                    covered = false;
                    break;
                }
            }

            if covered {
                count += 1;
            }
        }
        count
    }

    pub fn find_distress_beacon(&self) -> Option<Pair> {
        for loop_sensor in self.sensors.iter() {
            'second: for border_point in loop_sensor.edges_iter() {
                if !self.range_x.contains(&border_point.0)
                    || !self.range_y.contains(&border_point.1)
                {
                    continue;
                }

                for check_sensor in self.sensors.iter() {
                    if loop_sensor == check_sensor {
                        continue;
                    }
                    if check_sensor.covers(&border_point) {
                        continue 'second;
                    }
                }

                return Some(border_point);
            }
        }
        None
    }
}

pub fn day15a() -> usize {
    // let y_line = 10;
    let y_line = 2000000;
    let file = File::open("data/day15/day15a.txt").unwrap();

    let sensors = Sensors::from_file(file).unwrap();

    sensors.count_covered_row(y_line)
}

/// after looking on the internet for hints
pub fn day15b() -> usize {
    let file = File::open("data/day15/day15b.txt").unwrap();

    let mut sensors = Sensors::from_file(file).unwrap();
    // sensors.range_x = 0..=20;
    // sensors.range_y = 0..=20;

    sensors.range_x = 0..=4000000;
    sensors.range_y = 0..=4000000;

    if let Some(point) = sensors.find_distress_beacon() {
        return point.0 as usize * 4000000 + point.1 as usize;
    }

    1
}
