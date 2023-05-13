use crate::{snowflake, util};
use core::fmt::{self, Debug, Formatter};
use serde::Deserialize;

/// A struct representing a user's social links.
#[derive(Clone, Debug, Deserialize)]
pub struct Socials {
  /// A URL to this user's GitHub account.
  #[serde(default, deserialize_with = "util::deserialize_optional_string")]
  pub github: Option<String>,

  /// A URL to this user's Instagram account.
  #[serde(default, deserialize_with = "util::deserialize_optional_string")]
  pub instagram: Option<String>,

  /// A URL to this user's Reddit account.
  #[serde(default, deserialize_with = "util::deserialize_optional_string")]
  pub reddit: Option<String>,

  /// A URL to this user's Twitter account.
  #[serde(default, deserialize_with = "util::deserialize_optional_string")]
  pub twitter: Option<String>,

  /// A URL to this user's YouTube channel.
  #[serde(default, deserialize_with = "util::deserialize_optional_string")]
  pub youtube: Option<String>,
}

/// A struct representing a user logged into [Top.gg](https://top.gg).
#[derive(Clone, Deserialize)]
pub struct User {
  /// The Discord ID of this user.
  #[serde(deserialize_with = "snowflake::deserialize")]
  pub id: u64,

  /// The username of this user.
  pub username: String,

  #[deprecated(since = "1.1.0")]
  pub discriminator: String,

  /// The user's bio.
  #[serde(default, deserialize_with = "util::deserialize_optional_string")]
  pub bio: Option<String>,

  /// A URL to this user's profile banner image.
  #[serde(default, deserialize_with = "util::deserialize_optional_string")]
  pub banner: Option<String>,

  /// A struct of this user's social links.
  #[serde(rename = "social")]
  pub socials: Option<Socials>,

  /// Whether this user is a [Top.gg](https://top.gg) supporter or not.
  #[serde(rename = "supporter")]
  pub is_supporter: bool,

  /// Whether this user is a [Top.gg](https://top.gg) certified developer or not.
  #[serde(rename = "certifiedDev")]
  pub is_certified_dev: bool,

  /// Whether this user is a [Top.gg](https://top.gg) moderator or not.
  #[serde(rename = "mod")]
  pub is_moderator: bool,

  /// Whether this user is a [Top.gg](https://top.gg) website moderator or not.
  #[serde(rename = "webMod")]
  pub is_web_moderator: bool,

  /// Whether this user is a [Top.gg](https://top.gg) website administrator or not.
  #[serde(rename = "admin")]
  pub is_admin: bool,

  #[serde(default, deserialize_with = "util::deserialize_optional_string")]
  avatar: Option<String>,
}

impl User {
  /// Retrieves the Discord avatar URL of this user.
  ///
  /// It's format will be either PNG or GIF if animated.
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```rust,no_run
  /// use topgg::Client;
  ///
  /// #[tokio::main]
  /// async fn main() {
  ///   let token = env!("TOPGG_TOKEN").to_owned();
  ///   let client = Client::new(token);
  ///   
  ///   let user = client.get_user(661200758510977084u64).await.unwrap();
  ///   
  ///   println!("{}", user.avatar());
  /// }
  /// ```
  #[inline(always)]
  pub fn avatar(&self) -> String {
    util::get_avatar(&self.avatar, self.id)
  }
}

impl Debug for User {
  fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
    fmt
      .debug_struct("User")
      .field("id", &self.id)
      .field("username", &self.username)
      .field("bio", &self.bio)
      .field("banner", &self.banner)
      .field("socials", &self.socials)
      .field("is_supporter", &self.is_supporter)
      .field("is_certified_dev", &self.is_certified_dev)
      .field("is_moderator", &self.is_moderator)
      .field("is_web_moderator", &self.is_web_moderator)
      .field("is_admin", &self.is_admin)
      .field("avatar", &self.avatar())
      .finish()
  }
}

#[derive(Deserialize)]
pub(crate) struct Voted {
  pub(crate) voted: u8,
}

/// A struct representing a user who has voted on a Discord bot listed on [Top.gg](https://top.gg). (See [crate::Client::get_voters`])
#[derive(Clone, Debug, Deserialize)]
pub struct Voter {
  /// The Discord ID of this user.
  #[serde(deserialize_with = "snowflake::deserialize")]
  pub id: u64,

  /// The username of this user.
  pub username: String,

  avatar: Option<String>,
}

impl Voter {
  /// Retrieves the Discord avatar URL of this user.
  ///
  /// It's format will be either PNG or GIF if animated.
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```rust,no_run
  /// use topgg::Client;
  ///
  /// #[tokio::main]
  /// async fn main() {
  ///   let token = env!("TOPGG_TOKEN").to_owned();
  ///   let client = Client::new(token);
  ///   
  ///   for voter in client.get_voters().await.unwrap() {
  ///     println!("{}", voter.avatar());
  ///   }
  /// }
  /// ```
  #[must_use]
  pub fn avatar(&self) -> String {
    util::get_avatar(&self.avatar, self.id)
  }
}
