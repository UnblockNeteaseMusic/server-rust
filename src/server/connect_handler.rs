use hyper::{Body, Request, Response, Uri};

use crate::{error, ErrorResult};

fn host_addr(url: &Uri) -> ErrorResult<String> {
    url.authority()
        .map(|v| v.to_string())
        .ok_or(error::Error::InvalidRequest)
}

pub async fn connect_handler(_req: Request<Body>) -> ErrorResult<Response<Body>> {
    // let host_addr = host_addr(req.uri())?;
    //
    // tokio::task::spawn(async move {
    //     match hyper::upgrade::on(req).await {
    //         Ok(upgraded) => {
    //             todo!(); // Proxy Handler just like this:
    //                      // Refer to https://github.com/hyperium/hyper/blob/master/examples/http_proxy.rs
    //                      // if let Err(e) = tunnel(upgraded, addr).await {
    //                      //     eprintln!("server io error: {}", e);
    //                      // };
    //         }
    //         Err(e) => eprintln!("upgrade error: {}", e),
    //     }
    // });

    todo!()
}
