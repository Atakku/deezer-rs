use crate::genre::Genre;
use crate::track::Tracks;
use crate::user::Users;
use serde::Deserialize;
use surf::Client;

#[derive(Debug, Deserialize)]
pub struct Album {
  pub id: u64,
  pub title: String,
  pub upc: Option<String>,
  pub link: Option<String>,
  pub share: Option<String>,
  pub cover: String,
  pub cover_small: String,
  pub cover_medium: String,
  pub cover_big: String,
  pub cover_xl: String,
  pub genre_id: Option<i64>,
  pub genres: Option<Genres>,
  pub label: Option<String>,
  pub nb_tracks: Option<u32>,
  pub duration: Option<u32>,
  pub fans: Option<u32>,
  pub rating: Option<u32>,
  pub release_date: Option<String>,
  pub record_type: Option<String>,
  pub available: Option<bool>,
  pub tracklist: String,
  pub explicit_lyrics: Option<bool>,
  pub explicit_content_lyrics: Option<u32>,
  pub explicit_content_cover: Option<u32>,
  pub r#type: String,
  pub artist: Option<AlbumArtist>,
}

#[derive(Debug, Deserialize)]
pub struct AlbumArtist {
  pub id: u64,
  pub name: String,
}


#[derive(Debug, Deserialize)]
pub struct Genres {
  pub data: Vec<Genr>,
}

#[derive(Debug, Deserialize)]
pub struct Genr {
  pub id: i64,
  pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Albums {
  pub data: Vec<Album>,
}

pub struct AlbumService {
  client: Client,
}

impl AlbumService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }

  pub async fn get(&self, id: &str) -> Result<Album, surf::Error> {
    let res = self
      .client
      .get(format!("/album/{}", id))
      .recv_json::<Album>()
      .await?;
    Ok(res)
  }

  pub async fn get_fans(&self, id: &str) -> Result<Users, surf::Error> {
    let res = self
      .client
      .get(format!("/album/{}/fans", id))
      .recv_json::<Users>()
      .await?;
    Ok(res)
  }

  pub async fn get_tracks(&self, id: &str) -> Result<Tracks, surf::Error> {
    let res = self
      .client
      .get(format!("/album/{}/tracks", id))
      .recv_json::<Tracks>()
      .await?;
    Ok(res)
  }
}
