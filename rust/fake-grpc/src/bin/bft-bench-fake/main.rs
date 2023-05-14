pub mod pb {
    tonic::include_proto!("grpc.fake");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    todo!()
}
