use actix_web::{web, App, HttpServer, Responder, Error, HttpRequest, HttpResponse, http};
use serde::{Deserialize, Serialize};
use log::{debug, warn};
use futures::future::{ready, Ready};
use serde_json;
use actix_files as fs;

use crate::links::{LinksContainer, Link};

#[derive(Deserialize)]
struct QueryParams {
  query: String,
}

enum QueryResponse {
  BadQuery,
  Response(Vec<Link>)
}

#[derive(Serialize)]
enum GetUrlResponse {
  SlugDoesNotExist,
  #[serde(rename = "url")]
  Response(String)
}

// Responder
impl Responder for QueryResponse {
  type Error = Error;
  type Future = Ready<Result<HttpResponse, Error>>;

  fn respond_to(self, _req: &HttpRequest) -> Self::Future {
    let body = match self {
      QueryResponse::BadQuery => "{\"status\":\"Error\",\"reason\":\"Bad query\"}".to_owned(),
      QueryResponse::Response(res) => serde_json::to_string(&res).unwrap()
    };

    ready(Ok(HttpResponse::Ok()
      .header(http::header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
      .content_type("application/json")
      .body(body)))
  }
}

impl Responder for GetUrlResponse {
  type Error = Error;
  type Future = Ready<Result<HttpResponse, Error>>;

  fn respond_to(self, _req: &HttpRequest) -> Self::Future {
    let body = match self {
      GetUrlResponse::SlugDoesNotExist => "{\"status\":\"Error\",\"reason\":\"Slug does not exist\"}".to_owned(),
      GetUrlResponse::Response(res) => serde_json::to_string(&GetUrlResponse::Response(res)).unwrap()
    };

    ready(Ok(HttpResponse::Ok()
      .header(http::header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
      .content_type("application/json")
      .body(body)))
  }
}

async fn get_url(
  data: web::Data<LinksContainer>,
  p: web::Path<String>
 ) -> impl Responder {
  match data.get_ref().slugs_to_urls.get(p.as_ref()) {
    Some(val) => {
      debug!("Got slug request for {}", val);
      GetUrlResponse::Response(val.clone())
    },
    None => {
      warn!("Actix. Slug does not exist {}", p.as_ref());
      GetUrlResponse::SlugDoesNotExist
    }
  }
}

async fn get_query(
  data: web::Data<LinksContainer>,
  query_params: web::Query<QueryParams>
) -> impl Responder {
  if (*query_params).query.len() < 3 || (*query_params).query.len() > 50 {
    warn!("Actix. Bad query");
    QueryResponse::BadQuery
  } else {
    debug!("Got with q={}", &(*query_params).query);
    QueryResponse::Response(data.get_ref().filter_query(&(*query_params).query))
  }
}

async fn all_items(data: web::Data<LinksContainer>) -> impl Responder {
  let mut sorted_links = Vec::new();
  for link in data.get_ref().urls_to_links.values() {
    sorted_links.push(link.clone())
  }
  sorted_links.sort_by(|a, b| b.date.cmp(&a.date));
  QueryResponse::Response(sorted_links)
}

pub fn start_server(
  links_container: LinksContainer,
  url: String
) -> std::io::Result<()> {
  actix_rt::System::new("this-week-in-rust-search")
    .block_on(async move {
      HttpServer::new(move || {
        App::new()
          .data(links_container.clone())
          .route("/query", web::get().to(get_query))
          .route("/slug/{slug}", web::get().to(get_url))
          .route("/all", web::get().to(all_items))
          .service(fs::Files::new("/", "./build").index_file("index.html"))
      })
        .bind(&url)?
        .run()
        .await
    })
}