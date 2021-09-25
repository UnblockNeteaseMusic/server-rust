use crate::{Error, Result};
use reqwest::{self, header::HeaderMap, header::HeaderValue};
pub use reqwest::{Method, Proxy, Response, StatusCode};
use serde_json::json;
pub use serde_json::Value as Json;
pub use tokio::sync::oneshot::Receiver;
use url::Url;

static mut GLOBAL_PROXY: Option<Proxy> = None;
fn copy_global_proxy() -> Option<Proxy> {
	unsafe {
		match &GLOBAL_PROXY {
			None => None,
			Some(v) => Some(v.clone()),
		}
	}
}

pub fn setup_global_proxy(proxy: &Option<String>) -> Result<()> {
	match proxy {
		Some(p) => match Proxy::all(p) {
			Ok(pp) => {
				unsafe {
					GLOBAL_PROXY = Some(pp);
				}
				Ok(())
			}
			Err(e) => Err(Error::RequestFail(e)),
		},
		None => {
			unsafe {
				GLOBAL_PROXY = None;
			}
			Ok(())
		}
	}
}

// accept: 'application/json, text/plain, */*',
// 'accept-encoding': 'gzip, deflate',
// 'accept-language': 'zh-CN,zh;q=0.9',
// 'user-agent':
// 	'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/66.0.3359.181 Safari/537.36',
fn default_headers() -> HeaderMap {
	let mut headers = HeaderMap::new();
	headers.insert(
		"accept",
		HeaderValue::from_static("application/json, text/plain, */*"),
	);
	headers.insert("accept-languag", HeaderValue::from_static("zh-CN,zh;q=0.9"));
	return headers;
}

pub async fn request(
	method: Method,
	received_url: Url,
	received_headers: Option<Json>,
	body: Option<String>,
	proxy: Option<Proxy>,
	_cancel: Option<Box<dyn std::any::Any + 'static>>,
) -> Result<Response> {
	let mut _headers = received_headers.clone();
	let headers = _headers.get_or_insert(json!({})).as_object_mut();
	if headers.is_none() {
		return Err(Error::HeadersDataInvalid);
	}
	let proxy = proxy.or(copy_global_proxy());

	let mut client_builder = reqwest::Client::builder()
	.user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/66.0.3359.181 Safari/537.36")
	.gzip(true).deflate(true)
	.default_headers(default_headers());
	client_builder = match proxy {
		None => client_builder.no_proxy(),
		Some(p) => client_builder.proxy(p),
	};
	let client = client_builder.build().map_err(|e| Error::RequestFail(e))?;
	let mut client = client.request(method, received_url);

	for (key, val) in headers.unwrap() {
		match val.as_str() {
			None => {}
			Some(v) => client = client.header(key, v),
		};
	}

	if body.is_some() {
		client = client.body(body.unwrap());
	}
	let ans = client.send().await;
	ans.map_err(|e| Error::RequestFail(e))
}

#[cfg(test)]
mod test {
	use std::convert::Infallible;
	use std::future::Future;
	use std::net;
	use std::sync::mpsc as std_mpsc;
	use std::thread;
	use std::time::Duration;

	use super::*;
	use futures::stream::StreamExt;
	use tokio::runtime;
	use tokio::sync::oneshot;
	use tokio::test;

	#[test]
	async fn test_request_local_simple() {
		let server = http(move |_| async move { http::Response::default() });

		let url = format!("http://{}/1", server.addr());
		let res: Response = request(
			Method::GET,
			Url::parse(&url).unwrap(),
			None,
			None,
			None,
			None,
		)
		.await
		.unwrap();

		println!("{:#?}", res);
	}

	#[test]
	async fn test_request_headers() {
		let server = http(move |req| async move {
			assert!(req.headers()["custom-header"]
				.to_str()
				.unwrap()
				.contains("bababa"));

			println!("{:#?}", req);
			http::Response::default()
		});

		let url = format!("http://{}/1", server.addr());
		let res: Response = request(
			Method::GET,
			Url::parse(&url).unwrap(),
			Some(json!({"custom-header": "bababa"})),
			None,
			None,
			None,
		)
		.await
		.unwrap();

		assert_eq!(res.url().as_str(), &url);
		assert_eq!(res.status(), reqwest::StatusCode::OK);
		assert_eq!(res.remote_addr(), Some(server.addr()));
	}

	#[test]
	async fn test_request_body() {
		let server = http(move |mut req| async move {
			// assert!(req.body().to_str().unwrap().contains("bababa"));
			let mut full: Vec<u8> = Vec::new();
			while let Some(item) = req.body_mut().next().await {
				full.extend(&*item.unwrap());
			}

			println!("{:#?}", req);
			let body = String::from_utf8(full).unwrap();
			println!("{:#?}", body);
			assert_eq!("123456", body);
			http::Response::default()
		});

		let url = format!("http://{}/1", server.addr());
		let res: Response = request(
			Method::GET,
			Url::parse(&url).unwrap(),
			None,
			Some(String::from("123456")),
			None,
			None,
		)
		.await
		.unwrap();

		assert_eq!(res.url().as_str(), &url);
		assert_eq!(res.status(), reqwest::StatusCode::OK);
		assert_eq!(res.remote_addr(), Some(server.addr()));
	}

	#[test]
	async fn request_with_baidu() {
		let res: Response = request(
			Method::GET,
			Url::parse("https://baidu.com").unwrap(),
			None,
			None,
			None,
			None,
		)
		.await
		.unwrap();

		println!("{:#?}", res);
		// assert_eq!(res.url().as_str(), &url);
		assert_eq!(res.status(), reqwest::StatusCode::OK);
		// assert_eq!(res.remote_addr(), Some(server.addr()));
	}

	#[test]
	async fn test_request_proxy() {
		let url = "http://hyper.rs/prox";
		let server = http(move |req| {
			assert_eq!(req.method(), "GET");
			assert_eq!(req.uri(), url);
			assert_eq!(req.headers()["host"], "hyper.rs");

			async { http::Response::default() }
		});

		let proxy = format!("http://{}", server.addr());

		let res: Response = request(
			Method::GET,
			Url::parse(&url).unwrap(),
			None,
			None,
			Some(Proxy::http(&proxy).unwrap()),
			None,
		)
		.await
		.unwrap();

		println!("{:#?}", res);
		assert_eq!(res.url().as_str(), url);
		assert_eq!(res.status(), reqwest::StatusCode::OK);
	}

	struct Server {
		addr: net::SocketAddr,
		panic_rx: std_mpsc::Receiver<()>,
		shutdown_tx: Option<oneshot::Sender<()>>,
	}

	impl Server {
		fn addr(&self) -> net::SocketAddr {
			self.addr
		}
	}

	impl Drop for Server {
		fn drop(&mut self) {
			if let Some(tx) = self.shutdown_tx.take() {
				let _ = tx.send(());
			}

			if !::std::thread::panicking() {
				self
					.panic_rx
					.recv_timeout(Duration::from_secs(3))
					.expect("test server should not panic");
			}
		}
	}

	fn http<F, Fut>(func: F) -> Server
	where
		F: Fn(http::Request<hyper::Body>) -> Fut + Clone + Send + 'static,
		Fut: Future<Output = http::Response<hyper::Body>> + Send + 'static,
	{
		//Spawn new runtime in thread to prevent reactor execution context conflict
		thread::spawn(move || {
			let rt = runtime::Builder::new_current_thread()
				.enable_all()
				.build()
				.expect("new rt");
			let srv = rt.block_on(async move {
				hyper::Server::bind(&([127, 0, 0, 1], 0).into()).serve(hyper::service::make_service_fn(
					move |_| {
						let func = func.clone();
						async move {
							Ok::<_, Infallible>(hyper::service::service_fn(move |req| {
								let fut = func(req);
								async move { Ok::<_, Infallible>(fut.await) }
							}))
						}
					},
				))
			});

			let addr = srv.local_addr();
			let (shutdown_tx, shutdown_rx) = oneshot::channel();
			let srv = srv.with_graceful_shutdown(async move {
				let _ = shutdown_rx.await;
			});

			let (panic_tx, panic_rx) = std_mpsc::channel();
			let tname = format!(
				"test({})-support-server",
				thread::current().name().unwrap_or("<unknown>")
			);
			thread::Builder::new()
				.name(tname)
				.spawn(move || {
					rt.block_on(srv).unwrap();
					let _ = panic_tx.send(());
				})
				.expect("thread spawn");

			Server {
				addr,
				panic_rx,
				shutdown_tx: Some(shutdown_tx),
			}
		})
		.join()
		.unwrap()
	}
}
