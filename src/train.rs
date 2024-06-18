use std::fmt::{Display, Debug};
use crate::station::Stop;

pub struct Train {
    pub id: u32,
    pub stops: Vec<Stop>,
}
impl Display for Train {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Train({}){{{:?}}}", self.id, self.stops))
    }
}
impl Debug for Train {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Train({}){{{:?}}}", self.id, self.stops))
    }
}

pub struct TrainBuilder {
    id: u32,
    stops: Vec<Stop>,
}
impl TrainBuilder {
    pub fn new(id: u32) -> Self {
        Self { id, stops: vec![] }
    }
    #[allow(dead_code)]
    pub fn add_stop(self, stop: &Stop) -> Self {
        let mut stops = self.stops.clone();
        stops.push(stop.clone());
        Self { id: self.id, stops }
    }
    pub fn add_stops(self, old_stops: &Vec<Stop>) -> Self {
        let mut stops = self.stops.clone();
        stops.append(&mut old_stops.clone());
        Self { id: self.id, stops }
    }
    pub fn build(self) -> Train {
        Train {
            id: self.id,
            stops: self.stops,
        }
    }
}
