

#[derive(Debug, Clone)]
pub struct  Market {
    pub id: String,
    pub interval: String,
    pub limit: u32
}

impl Market {
    pub fn new(id: String, interval: String, limit: u32) -> Self {
        Market { id, interval, limit }
    }
}
