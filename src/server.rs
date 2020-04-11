use actix_web::{web, App, HttpServer, Responder};
use serde::Deserialize;
use log::{info};

use crate::links::LinksContainer;

#[derive(Deserialize)]
struct QueryParams {
  query: String,
}

async fn index(
  data: web::Data<LinksContainer>,
  query_params: web::Query<QueryParams>
) -> impl Responder {
  info!("Get with q={}", &(*query_params).query);
  data.get_ref().filter_query(&(*query_params).query)
}

pub fn start_server(
  links_container: LinksContainer,
  url: String
) -> std::io::Result<()> {
  actix_rt::System::new("server")
    .block_on(async move {
      HttpServer::new(move || {
        App::new()
          .data(links_container.clone())
          .route("/query", web::get().to(index))
      })
        .bind(&url)?
        .run()
        .await
    })
}