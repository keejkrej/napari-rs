#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Stat {
    pub min: i64,
    pub max: i64,
    pub sum: i64,
    pub count: u64,
}

impl Stat {
    pub fn new(value: i64) -> Self {
        Self {
            min: value,
            max: value,
            sum: value,
            count: 1,
        }
    }

    pub fn add(&mut self, value: i64) {
        self.sum += value;
        self.count += 1;
        self.max = self.max.max(value);
        self.min = self.min.min(value);
    }

    pub fn average(&self) -> i64 {
        div_floor(self.sum, self.count as i64)
    }
}

fn div_floor(numerator: i64, denominator: i64) -> i64 {
    let quotient = numerator / denominator;
    let remainder = numerator % denominator;
    if remainder != 0 && ((remainder > 0) != (denominator > 0)) {
        quotient - 1
    } else {
        quotient
    }
}
