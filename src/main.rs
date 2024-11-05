#![allow(non_snake_case)]

use dioxus::prelude::*;
use router::App;
mod router;

#[dotenvy::load]
fn main() {
	// #[cfg(not(feature = "server"))]
	// dioxus::fullstack::prelude::server_fn::client::set_server_url("http://127.0.0.1:8080");

	dioxus::LaunchBuilder::new().with_cfg(web!(dioxus::web::Config::new().hydrate(true))).launch(App);
}
