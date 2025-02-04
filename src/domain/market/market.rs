#[derive(Debug, Clone)]
pub struct Market {
    pub id: String,
    pub interval: Interval,
    pub limit: u32,
}

#[derive(Debug, Clone)]
pub enum Interval {
    D1,
    H1,
}

impl Market {
    pub fn new(id: String, interval: Interval, limit: u32) -> Self {
        Market {
            id,
            interval,
            limit,
        }
    }
}
