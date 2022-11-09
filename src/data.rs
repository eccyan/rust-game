use serde::Deserialize;
use std::fs::File;

#[derive(Debug, Deserialize)]
struct Feature {
    location: Location,
    name: String,
}

#[derive(Debug, Deserialize)]
struct Location {
    latitude: f32,
    longitude: f32,
}

#[allow(dead_code)]
pub fn load() -> Vec<crate::world::Feature> {
    let file = File::open("examples/data/route_guide_db.json").expect("failed to open data file");

    let decoded: Vec<Feature> =
        serde_json::from_reader(&file).expect("failed to deserialize features");

    decoded
        .into_iter()
        .map(|feature| crate::world::Feature {
            name: feature.name,
            location: Some(crate::world::Point {
                longitude: feature.location.longitude,
                latitude: feature.location.latitude,
            }),
        })
        .collect()
}