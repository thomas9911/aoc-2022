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
}

#[derive(Debug, PartialEq, Clone)]
struct Sensors {
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
}

pub fn day15a() -> usize {
    // let y_line = 10;
    let y_line = 2000000;
    let file = File::open("data/day15/day15a.txt").unwrap();

    let sensors = Sensors::from_file(file).unwrap();

    sensors.count_covered_row(y_line)
}

pub fn day15b() -> usize {
    let file = File::open("data/day15/day15b.txt").unwrap();

    let mut sensors = Sensors::from_file(file).unwrap();

    sensors.range_x = RangeInclusive::new(0, 20);
    sensors.range_y = RangeInclusive::new(0, 20);
    // dbg!(sensors);
    // sensors.print_covered_row(9);
    for y in sensors.range_y.clone() {
        // println!("{}: {}", y, sensors.count_covered_row(y));
        // sensors.print_covered_row(y);
    }
    // sensors.count_covered_row(y_line)
    1
}
