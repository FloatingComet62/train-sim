use trains::{station::{Station, Stop, stops_builder}, train::{TrainBuilder, Train}};

fn main() {
    let stations: Vec<Station> = (0..5).map(|n| Station(n)).collect();
    let stops: Vec<Vec<Stop>> = vec![0, 150, 250]
        .iter()
        .map(|&n| stops_builder(&stations, n, &vec![100; 10]).unwrap())
        .collect();
    let trains: Vec<Train> = vec![0, 1, 2]
        .iter()
        .map(|&n| TrainBuilder::new(n).add_stops(&stops[n as usize]).build())
        .collect();
    println!("Hello, world!\n{:#?}", trains);
}
