use trains::{
    station::{stops_builder, Station, Stop},
    train::{Train, TrainBuilder},
};

fn main() {
    let mut time: u64 = 0;

    let stations: Vec<Station> = (0..5).map(|n| Station(n)).collect();
    let stops: Vec<Vec<Stop>> = vec![0, 3, 5]
        .iter()
        .map(|&n| stops_builder(&stations, n, &vec![2; 10]).unwrap())
        .collect();
    let mut trains: Vec<Train> = vec![0, 1, 2]
        .iter()
        .map(|&n| TrainBuilder::new(n).add_stops(&stops[n as usize]).build())
        .collect();

    loop {
        if trains.iter().all(|train| train.reached_end()) && time != 0 {
            break;
        }

        trains.iter_mut().for_each(|train| train.update_state(time));
        println!("{} {:#?}", time, trains);
        time += 1;
    }
}
