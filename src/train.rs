use crate::station::{Station, Stop};
use std::fmt::{Debug, Display};

#[derive(PartialEq)]
pub struct MovingState {
    pub from: Station,
    pub to: Station,
}
impl Display for MovingState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} to {}", self.from.0, self.to.0))
    }
}

#[derive(PartialEq)]
pub enum TrainState {
    Off,
    Moving(MovingState),
    Resting(Station),
}
impl Display for TrainState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TrainState::Off => f.write_str("Off"),
            TrainState::Resting(station) => f.write_fmt(format_args!("Station({})", station.0)),
            TrainState::Moving(moving_state) => {
                f.write_fmt(format_args!("Moving({})", moving_state))
            }
        }
    }
}

pub struct Train {
    pub id: u32,
    pub stops: Vec<Stop>,
    pub stop_index: usize,
    pub state: TrainState,
}
impl Train {
    pub fn new(id: u32, stops: Vec<Stop>) -> Self {
        Self {
            id,
            stops,
            stop_index: 0,
            state: TrainState::Off,
        }
    }
    fn get_stop(&self, i: &usize) -> &Stop {
        &self.stops[i / 2]
    }
    pub fn reached_end(&self) -> bool {
        self.stop_index >= 2 * self.stops.len()
    }
    pub fn update_state(&mut self, time: u64) {
        if self.state == TrainState::Off {
            if self.reached_end() {
                return;
            }
            let stop = self.get_stop(&self.stop_index);
            if stop.arrival_unixtimestamp < time {
                self.state = TrainState::Resting(stop.station.clone());
            }
            return;
        } else if self.reached_end() {
            self.state = TrainState::Off;
            return;
        }
        if self.stop_index % 2 == 0 {
            let stop = self.get_stop(&self.stop_index).clone();
            // at the station
            if stop.departure_unixtimestamp > time {
                return;
            }
            // depart the station
            self.stop_index += 1;
            if (self.stop_index + 1) >= 2 * self.stops.len() {
                self.state = TrainState::Off;
                self.stop_index += 1;
                return;
            }
            self.state = TrainState::Moving(MovingState {
                from: stop.station.clone(),
                to: self.get_stop(&(self.stop_index + 1)).station.clone(),
            });
            return;
        }

        let stop = self.get_stop(&(self.stop_index + 1)).clone();
        if stop.arrival_unixtimestamp < time {
            // depoart at station
            self.stop_index += 1;
            self.state = TrainState::Resting(stop.station);
        }
    }
}
impl Display for Train {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Train({})[{};{}]",
            self.id, self.state, self.stop_index
        ))
    }
}
impl Debug for Train {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Train({})[{};{}]",
            self.id, self.state, self.stop_index
        ))
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
        Train::new(self.id, self.stops)
    }
}
