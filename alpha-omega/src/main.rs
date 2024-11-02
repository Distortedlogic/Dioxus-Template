use dioxus::prelude::*;
use dotenvy::dotenv;

const _: Asset = asset!("public/tailwind.css");
const _: Asset = asset!("public/data/upwork.parquet");

fn main() {
	dotenv().ok();

	#[cfg(not(feature = "server"))]
	dioxus::fullstack::prelude::server_fn::client::set_server_url("http://127.0.0.1:8001");

	#[cfg(feature = "desktop")]
	{
		use app::App;
		use db::create_pool;
		use dioxus::desktop::{Config, WindowBuilder};
		use std::env;

		dioxus::LaunchBuilder::desktop()
			.with_cfg(
				Config::new()
					.with_prerendered({
						let pre_rendered_dom = VirtualDom::prebuilt(App);
						dioxus::ssr::pre_render(&pre_rendered_dom)
					})
					.with_window(WindowBuilder::new().with_resizable(true))
					.with_custom_index(include_str!("index.html").to_string()),
			)
			.with_context(server_only!(create_pool(env::var("DATABASE_URL").unwrap().as_str())))
			.launch(App)
	}

	#[cfg(feature = "mobile")]
	{
		use app::App;
		use dioxus::mobile::Config;

		#[cfg(target_os = "android")]
		{
			use dioxus::mobile::wry::android_binding;
			fn init_logging() {
				android_logger::init_once(android_logger::Config::default().with_min_level(log::Level::Trace).with_tag("mobile-demo"));
			}
			init_logging();
		}

		#[cfg(any(target_os = "android", target_os = "ios"))]
		{
			fn stop_unwind<F: FnOnce() -> T, T>(f: F) -> T {
				match std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
					Ok(t) => t,
					Err(err) => {
						eprintln!("attempt to unwind out of `rust` with err: {:?}", err);
						std::process::abort()
					},
				}
			}

			fn _start_app() {
				stop_unwind(|| main().unwrap());
			}

			#[no_mangle]
			#[inline(never)]
			pub extern "C" fn start_app() {
				#[cfg(target_os = "android")]
				android_binding!(com_example, mobile_demo, _start_app);
				#[cfg(target_os = "ios")]
				_start_app()
			}
		}

		dioxus::LaunchBuilder::mobile().with_cfg(Config::default().with_custom_index(include_str!("index.html").to_string())).launch(App);
	}

	#[cfg(feature = "web")]
	{
		use app::App;
		use db::create_pool;
		use std::env;
		use tracing_wasm;

		tracing_wasm::set_as_global_default();
		dioxus::LaunchBuilder::web().with_context(create_pool(env::var("DATABASE_URL").unwrap().as_str())).launch(App);
	}

	#[cfg(feature = "server")]
	{
		use app::App;
		use axum::routing::*;
		use db::create_pool;
		use std::env;
		use std::sync::Arc;

		let runtime = tokio::runtime::Runtime::new().unwrap();
		runtime.block_on(async {
			tracing_subscriber::fmt().with_max_level(tracing::Level::INFO).init();

			let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8001));
			let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
			let cfg = ServeConfig::builder().index_html(include_str!("index.html").to_string()).build().unwrap();

			axum::serve(
				listener,
				Router::new()
					.register_server_functions_with_context(Arc::new(vec![Box::new(|| Box::new(create_pool(env::var("DATABASE_URL").unwrap().as_str())))]))
					.fallback(get(render_handler).with_state(RenderHandleState::new(cfg.clone(), App).with_ssr_state(SSRState::new(&cfg))))
					.into_make_service(),
			)
			.await
			.unwrap();
		});
	}
}
