use app::App;
use db::create_pool;
use dioxus::prelude::*;
use std::env;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
mod dx_logger;

#[dotenvy::load(path = "../.env")]
fn main() {
	#[cfg(target_os = "android")]
	{
		android_logger::init_once(android_logger::Config::default().with_min_level(log::Level::Trace).with_tag("mobile-demo"));
	}

	#[cfg(not(any(target_os = "android", target_os = "ios")))]
	{
		use dx_logger as dioxus_logger;
		dioxus_logger::init(dioxus_logger::tracing::Level::INFO).expect("failed to init logger");
	}
	client!(dioxus::fullstack::prelude::server_fn::client::set_server_url("http://127.0.0.1:8001"));

	dioxus::LaunchBuilder::new()
		.with_cfg(web!(dioxus::web::Config::new().hydrate(true)))
		.with_cfg(server_only!(ServeConfigBuilder::new()))
		.with_cfg(desktop!(dioxus::desktop::Config::new()
			.with_prerendered({
				let pre_rendered_dom = VirtualDom::prebuilt(App);
				dioxus::ssr::pre_render(&pre_rendered_dom)
			})
			.with_background_color((255, 255, 255, 1))
			.with_window(dioxus::desktop::WindowBuilder::new().with_resizable(true).with_minimizable(true).with_maximizable(true).with_closable(true))))
		.with_cfg(mobile!(dioxus::desktop::Config::new()
			.with_prerendered({
				let pre_rendered_dom = VirtualDom::prebuilt(App);
				dioxus::ssr::pre_render(&pre_rendered_dom)
			})
			.with_background_color((255, 255, 255, 1))
			.with_window(dioxus::desktop::WindowBuilder::new().with_resizable(true).with_minimizable(true).with_maximizable(true).with_closable(true))))
		.with_context(server_only!(create_pool(env::var("DATABASE_URL").unwrap().as_str())))
		.launch(App);
}
