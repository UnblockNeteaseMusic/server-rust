use std::{
    convert::Infallible, future::Future, net, sync::mpsc as std_mpsc, thread, time::Duration,
};

use reqwest::{Method, Proxy, Response};
use serde_json::json;
use tokio::{runtime, sync::oneshot, test};
use url::Url;

use futures::stream::StreamExt;

use crate::request::{proxy_manager::ProxyManager, request};

#[test]
async fn test_request_local_simple() {
    let server = http(move |_| async move { http::Response::default() });

    let url = format!("http://{}/1", server.addr());
    let res: Response = request(Method::GET, Url::parse(&url).unwrap(), None, None, None)
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
    )
    .await
    .unwrap();

    assert_eq!(res.url().as_str(), &url);
    assert_eq!(res.status(), reqwest::StatusCode::OK);
    assert_eq!(res.remote_addr(), Some(server.addr()));
}

#[test]
async fn request_with_example_com() {
    let res: Response = request(
        Method::GET,
        Url::parse("https://example.com").unwrap(),
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

    let proxy = Proxy::http(format!("http://{}", server.addr())).unwrap();
    let proxy_manager = ProxyManager { proxy: Some(proxy) };

    let res: Response = request(
        Method::GET,
        Url::parse(url).unwrap(),
        None,
        None,
        Some(&proxy_manager),
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
            self.panic_rx
                .recv_timeout(Duration::from_secs(3))
                .expect("tests server should not panic");
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

        #[allow(clippy::async_yields_async)]
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
            "tests({})-support-server",
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
