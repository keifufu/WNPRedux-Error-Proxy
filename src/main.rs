use std::{collections::HashMap, sync::Arc};

use config::Config;
use rocket::{
  http::Status,
  serde::{json::Json, Deserialize},
  tokio::sync::Mutex,
  State,
};
use webhook::client::WebhookClient;

#[macro_use]
extern crate rocket;

#[derive(Debug)]
pub struct AppConfig {
  port: i64,
  webhook_url: String,
  webhook_avatar_url: String,
}

struct Cache {
  data: HashMap<String, bool>,
}

impl Cache {
  fn new() -> Self {
    Cache {
      data: HashMap::new(),
    }
  }

  fn contains(&self, key: &str) -> bool {
    self.data.contains_key(key)
  }

  fn insert(&mut self, key: String, value: bool) {
    self.data.insert(key, value);
  }
}

#[derive(Debug, Deserialize)]
enum ReportType {
  #[serde(rename = "manual")]
  Manual,
  #[serde(rename = "automatic")]
  Automatic,
}

#[derive(Debug, Deserialize)]
struct ReportBody {
  #[serde(rename = "type")]
  report_type: ReportType,
  message: String,
  #[serde(rename = "extVersion")]
  ext_version: String,
}

#[post("/report", data = "<body>")]
async fn report_route(
  body: Json<ReportBody>,
  config: &State<AppConfig>,
  cache: &State<Arc<Mutex<Cache>>>,
) -> Result<String, Status> {
  let body = body.into_inner();

  if matches!(body.report_type, ReportType::Automatic) {
    let msg = format!("{} - {}", body.ext_version, body.message);
    let mut cache = cache.lock().await;
    if cache.contains(&msg) {
      return Ok("OK".to_string());
    }
    cache.insert(msg.clone(), true);
  }

  let client: WebhookClient = WebhookClient::new(&config.webhook_url);
  if client
    .send(|message| {
      message
        .username("WNPRedux Reporter")
        .avatar_url(&config.webhook_avatar_url)
        .embed(|embed| {
          embed
            .title(if matches!(body.report_type, ReportType::Automatic) {
              "Automatic Report"
            } else {
              "Manual Report"
            })
            .description(&body.message)
            .footer(&format!("v{}", &body.ext_version), None)
        })
    })
    .await
    .is_err()
  {
    return Err(Status::InternalServerError);
  };

  Ok("OK".to_string())
}

#[launch]
fn rocket() -> _ {
  let config = Config::builder()
    .add_source(config::File::with_name("config.toml"))
    .build()
    .unwrap();

  let config = AppConfig {
    port: config.get_int("port").unwrap(),
    webhook_url: config.get_string("webhook-url").unwrap(),
    webhook_avatar_url: config.get_string("webhook-avatar-url").unwrap(),
  };

  let cache = Arc::new(Mutex::new(Cache::new()));

  let figment = rocket::Config::figment()
    .merge(("port", config.port))
    .merge(("address", "0.0.0.0"));

  rocket::custom(figment)
    .mount("/", routes![report_route])
    .manage(config)
    .manage(cache)
}
