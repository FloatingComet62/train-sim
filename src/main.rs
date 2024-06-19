use trains::{
    log,
    station::{stops_builder, Station},
    train::{Train, TrainBuilder},
};

static MAX_ITERATIONS: u64 = 65535;

fn main() {
    let mut time: u64 = 0;

    let stations: Vec<Station> = (0..5).map(|n| Station(n)).collect();
    let mut trains: Vec<Train> = vec![0, 3, 5]
        .iter()
        .enumerate()
        .map(|(n, &m)| {
            TrainBuilder::new(n as u32)
                .add_stops(&stops_builder(&stations, m, &vec![2; 10]).unwrap())
                .build()
        })
        .collect();

    loop {
        if time > MAX_ITERATIONS {
            log!(err "Fail save triggered");
            break;
        }
        if trains.iter().all(|train| train.reached_end()) {
            break;
        }

        trains.iter_mut().for_each(|train| train.update_state(time));
        println!("{} {:#?}", time, trains);
        time += 1;
    }
}
