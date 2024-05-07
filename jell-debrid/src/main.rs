use clients::jellyfin::client::JellyfinClient;
use clients::seerrs::client::SeerrClient;
use clients::trakt::client::{TraktClient, TraktOAuthToken};

use env_logger;
use figment::{providers::Env, Figment};
use serde::Deserialize;
use tokio::time;

#[derive(Debug, Default, Deserialize, PartialEq, Eq)]
struct AppConfig {
    jf_api_key: String,
    rd_api_key: String,
    seerr_api_key: String,

    trakt_api_key: Option<String>,
    trakt_client_id: String,
    trakt_client_secret: Option<String>,
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let cfg: AppConfig = Figment::new().merge(Env::raw()).extract().unwrap();

    /*
    let jellyfin = JellyfinClient::new("http://192.168.0.69:8096", &cfg.jf_api_key);
    let response = jellyfin.get_system_info().await;
    match response {
        Ok(v) => println!(" in main deserialized = {:?}", v),
        Err(e) => println!("in main error: {e:?}"),
    }

    let response = jellyfin.refresh_libraries().await;
    match response {
        Ok(v) => println!(" in main deserialized = {:?}", v),
        Err(e) => println!("in main error: {e:?}"),
    }
    */
    let token = if cfg.trakt_api_key.is_none() {
        let token =
            TraktClient::oauth2(&cfg.trakt_client_id, &cfg.trakt_client_secret.unwrap()).await;
        println!("Got new trakt token = {:?}", token.access_token);
        token.access_token
    } else {
        cfg.trakt_api_key.unwrap()
    };

    let trakt = TraktClient::new(&token, &cfg.trakt_client_id);
    let response = trakt.get_watchlist().await;
    match response {
        Ok(v) => println!(" in main deserialized = {:?}", v),
        Err(e) => println!("in main error: {e:?}"),
    }

    /*
    let seerr = SeerrClient::new("http://192.168.0.69:5055", &cfg.seerr_api_key);

    let mut interval = time::interval(time::Duration::from_secs(10));
    loop {
        interval.tick().await;

        let response = seerr.get_unfulfilled_requests().await;
        match response {
            Ok(v) => println!("in main seer deserialized = {:?}", v),
            Err(e) => println!("in main seer error: {e:?}"),
        }
    }
    */
}
