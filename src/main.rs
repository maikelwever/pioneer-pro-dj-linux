#[macro_use]
extern crate clap;

mod rekordbox;
mod utils;
mod component;
mod rpc;
mod library;

use component::App;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = clap_app!(myapp => 
        (@arg LIBRARY_PATH: +required "Path to music library to serve")
    ).get_matches();

    let library_path = matches.value_of("LIBRARY_PATH").unwrap();
    let mut app = App::new(library_path);
    app.run().await;

    Ok(())
}
