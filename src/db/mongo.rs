use mongodb::{
    error::Result,
    options::{ClientOptions, Compressor},
    Client,
};

pub async fn db_connection() -> Result<Client> {
    let mongo_uri: String = "mongodb://127.0.0.1".to_string();

    let mut client_options = ClientOptions::parse(mongo_uri).await.unwrap();
    client_options.compressors = Some(vec![
        Compressor::Snappy,
        Compressor::Zlib {
            level: Default::default(),
        },
        Compressor::Zstd {
            level: Default::default(),
        },
    ]);

    return Client::with_options(client_options);
}