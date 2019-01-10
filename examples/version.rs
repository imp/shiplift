use shiplift::Docker;
use tokio::prelude::Future;

fn main() {
    let docker = Docker::new();
    tokio::run(
        docker
            .version()
            .map(|version| println!("Docker version: {:#?}", version))
            .map_err(|e| eprintln!("Error: {}", e)),
    );
}
