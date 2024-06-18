use std::fmt::{Debug, Display};

#[derive(Clone, PartialEq)]
pub struct Station(pub u32);

#[derive(Clone)]
pub struct Stop {
    pub station: Station,
    pub arrival_unixtimestamp: u64,
    pub departure_unixtimestamp: u64,
}
impl Stop {
    pub fn new(station: Station, arrival: u64, departure: u64) -> Self {
        Self {
            station,
            arrival_unixtimestamp: arrival,
            departure_unixtimestamp: departure,
        }
    }
}
impl Display for Stop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Stop({}  {} to {})",
            self.station.0, self.arrival_unixtimestamp, self.departure_unixtimestamp
        ))
    }
}
impl Debug for Stop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Stop({}  {} to {})",
            self.station.0, self.arrival_unixtimestamp, self.departure_unixtimestamp
        ))
    }
}

pub fn stops_builder(
    stations: &Vec<Station>,
    start_time: u64,
    intervals: &Vec<u64>,
) -> Option<Vec<Stop>> {
    if stations.len() < intervals.len() / 2 {
        return None;
    }

    let mut stops: Vec<Stop> = vec![];
    intervals
        .iter()
        .enumerate()
        .fold(start_time, |acc, (i, x)| {
            if i % 2 != 0 {
                return acc + x;
            }
            stops.push(Stop::new(stations[i / 2].clone(), acc, acc + x));
            acc + x
        });
    Some(stops)
}
