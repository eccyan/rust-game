use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Instant;

use futures::{Stream, StreamExt};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status};

use world::world_server::{World, WorldServer};
use world::{Point, Feature};

pub mod world {
    tonic::include_proto!("world");
}

mod data;

#[derive(Default)]
pub struct WorldService {
    features: Arc<Vec<Feature>>,
}

#[tonic::async_trait]
impl World for WorldService {
    async fn get_feature(&self, request: Request<Point>) -> Result<Response<Feature>, Status> {
        println!("GetFeature = {:?}", request);

        for feature in &self.features[..] {
            if feature.location.as_ref() == Some(request.get_ref()) {
                return Ok(Response::new(feature.clone()));
            }
        }

        Ok(Response::new(Feature::default()))
    }
    
    type GetAroundFeaturesStream = ReceiverStream<Result<Feature, Status>>;
    
    async fn get_around_features(
        &self,
        request: Request<Point>,
    ) -> Result<Response<Self::GetAroundFeaturesStream>, Status> {
        println!("ListFeatures {:?}", request);
        
        let (tx, rx) = mpsc::channel(4);
        let features = self.features.clone();

        tokio::spawn(async move {
            for feature in &features[..] {
                if in_around(feature.location.as_ref().unwrap(), request.get_ref()) {
                    println!("  => send {:?}", feature);
                    tx.send(Ok(feature.clone())).await.unwrap();
                }
            }

            println!(" /// done sending");
        });

        Ok(Response::new(ReceiverStream::new(rx)))    
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:10000".parse().unwrap();

    println!("WorldServer listening on: {}", addr);

    let world = WorldService {
        features: Arc::new(data::load()),
    };

    let svc = WorldServer::new(world);

    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}

impl Eq for Point {}

fn in_around(feature_location: &Point, point: &Point) -> bool {
    true
}
