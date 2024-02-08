use bytes::Bytes;
use http_body_util::Full;
use hyper::service::Service;
use hyper::{body::Incoming as IncomingBody, Request, Response};
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

type Counter = i32;

#[derive(Debug, Clone)]
pub struct Svc {
    counter: Arc<Mutex<Counter>>,
}

impl Svc {
    pub fn new() -> Self {
        Svc {
            counter: Arc::new(Mutex::new(0)),
        }
    }
}

impl Default for Svc {
    fn default() -> Self {
        Self::new()
    }
}

impl Service<Request<IncomingBody>> for Svc {
    type Response = Response<Full<Bytes>>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: Request<IncomingBody>) -> Self::Future {
        fn mk_response(s: String) -> Result<Response<Full<Bytes>>, hyper::Error> {
            Ok(Response::builder().body(Full::new(Bytes::from(s))).unwrap())
        }

        if req.uri().path() != "/favicon.ico" {
            *self.counter.lock().expect("lock poisoned") += 1;
        }

        let res = match req.uri().path() {
            "/" => mk_response(format!("home! counter = {:?}", self.counter)),
            "/posts" => mk_response(format!("posts, of course! counter = {:?}", self.counter)),
            "/authors" => mk_response(format!(
                "authors extraordinare! counter = {:?}",
                self.counter
            )),
            // Return the 404 Not Found for other routes, and don't increment counter.
            _ => return Box::pin(async { mk_response("oh no! not found".into()) }),
        };

        Box::pin(async { res })
    }
}
