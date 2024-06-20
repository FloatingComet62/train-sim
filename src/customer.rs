use crate::{
    station::{Station, Stop},
    train::Train,
};

pub struct Customer {
    pub at: Station,
    pub goto: Station,
    pub arrival_time: Option<u64>,
    pub allotted_train: Option<Train>,
}

impl Customer {
    pub fn new(at: Station, goto: Station, arrival_time: Option<u64>) -> Self {
        Self {
            at,
            goto,
            arrival_time,
            allotted_train: None,
        }
    }
    pub fn get_allottedable_trains(&self, trains: &Vec<Train>) -> Vec<usize> {
        let check = |upcoming_stop: &Stop,
                     will_come_to_current_station: &mut bool,
                     will_goto_required_station: &mut bool|
         -> bool {
            if upcoming_stop.station == self.at {
                *will_come_to_current_station = true;
            }
            if upcoming_stop.station == self.goto {
                *will_goto_required_station = true;
                // the reason for returning is to discard the trains which will move in
                // the opposite direction, aka moving from "goto" to "at"
                return true;
            }
            false
        };
        let filter = |(i, train): (usize, &Train)| {
            let upcoming_stops = train.upcoming_stops(self.arrival_time.unwrap_or(0));
            let mut will_come_to_current_station = false;
            let mut will_goto_required_station = false;
            for upcoming_stop in upcoming_stops {
                if check(
                    &upcoming_stop,
                    &mut will_come_to_current_station,
                    &mut will_goto_required_station,
                ) {
                    break;
                }
            }
            if will_come_to_current_station && will_goto_required_station {
                return Some(i);
            }
            None
        };
        trains.iter().enumerate().filter_map(filter).collect()
    }
}

pub struct CustomerBuilder {
    pub at: Station,
    pub goto: Option<Station>,
    pub arrival_time: Option<u64>,
    pub allotted_train: Option<Train>,
}
#[derive(Debug)]
pub enum BuildFail {
    MissingGotoProperty,
}
impl CustomerBuilder {
    pub fn new(at: Station) -> Self {
        Self {
            at,
            goto: None,
            arrival_time: None,
            allotted_train: None,
        }
    }
    pub fn new_u32(at: u32) -> Self {
        Self {
            at: Station(at),
            goto: None,
            arrival_time: None,
            allotted_train: None,
        }
    }
    pub fn goto(mut self, goto: Station) -> Self {
        self.goto = Some(goto);
        self
    }
    pub fn goto_u32(mut self, goto: u32) -> Self {
        self.goto = Some(Station(goto));
        self
    }
    pub fn arrival_time(mut self, arrival_time: u64) -> Self {
        self.arrival_time = Some(arrival_time);
        self
    }
    pub fn build(self) -> Result<Customer, BuildFail> {
        match self.goto {
            None => Err(BuildFail::MissingGotoProperty),
            Some(goto) => Ok(Customer::new(self.at, goto, self.arrival_time)),
        }
    }
}
