use std::convert::TryInto;
use std::time::Duration;
use surf::{Client, Config, Url};

pub mod album;
pub mod artist;
pub mod chart;
pub mod editorial;
pub mod genre;
pub mod infos;
pub mod options;
pub mod playlist;
pub mod radio;
pub mod search;
pub mod track;
pub mod user;

pub struct Deezer {
    pub album: album::AlbumService,
    pub artist: artist::ArtistService,
    pub chart: chart::ChartService,
    pub editorial: editorial::EditorialService,
    pub genre: genre::GenreService,
    pub infos: infos::InfosService,
    pub options: options::OptionsService,
    pub playlist: playlist::PlaylistService,
    pub radio: radio::RadioService,
    pub search: search::SearchService,
    pub track: track::TrackService,
    pub user: user::UserService,
}

const BASE_URL: &str = "https://api.deezer.com/";

impl Deezer {
    pub fn from_client(client: Client) -> Self {
        Self {
            album: album::AlbumService::new(&client),
            artist: artist::ArtistService::new(&client),
            chart: chart::ChartService::new(&client),
            editorial: editorial::EditorialService::new(&client),
            genre: genre::GenreService::new(&client),
            infos: infos::InfosService::new(&client),
            options: options::OptionsService::new(&client),
            playlist: playlist::PlaylistService::new(&client),
            radio: radio::RadioService::new(&client),
            search: search::SearchService::new(&client),
            track: track::TrackService::new(&client),
            user: user::UserService::new(&client),
        }
    }

    pub fn new() -> Self {
        let retry = RetryMiddleware::new(
            5,
            ExponentialBackoff::builder().build_with_max_retries(5),
            2,
        );
        let client: Client = Config::new().with(retry)
            .set_base_url(Url::parse(BASE_URL).unwrap())
            .set_timeout(Some(Duration::from_secs(5)))
            .try_into()
            .unwrap();
        Self::from_client(client)
    }
}
