use std::sync::{atomic::AtomicUsize, Arc};

mod my {
    // use std::thread::spawn;

    pub trait MyService<Request> {
        type Response;
        type Error;

        async fn run(&mut self, req: Request) -> Result<Self::Response, Self::Error>;
    }

    pub struct MyApp;
    pub struct MyAppFactory;

    pub struct MyConnectionInfo;

    impl MyService<MyConnectionInfo> for MyAppFactory {
        type Response = MyApp;
        type Error = anyhow::Error;

        async fn run(&mut self, _req: MyConnectionInfo) -> Result<Self::Response, Self::Error> {
            Ok(MyApp)
        }
    }

    pub struct MyRequest;
    pub struct MyResponse;

    impl MyService<MyRequest> for MyApp {
        type Response = MyResponse;
        type Error = anyhow::Error;

        async fn run(&mut self, _req: MyRequest) -> Result<Self::Response, Self::Error> {
            Ok(MyResponse)
        }
    }
}

mod http {
    use std::collections::HashMap;

    #[derive(Clone, Debug)]
    pub struct ConnInfo {
        pub host_and_port: String,
    }

    pub struct Request {
        pub path_and_query: String,
        pub headers: HashMap<String, String>,
        pub body: Vec<u8>,
    }

    #[derive(Debug)]
    pub struct Response {
        pub status: u32,
        pub headers: HashMap<String, String>,
        pub body: Vec<u8>,
    }
}

mod fakeserver {
    use std::collections::HashMap;
    use tower::{Service, ServiceExt};

    pub async fn run<AppFactory, App>(mut app_factory: AppFactory)
    where
        AppFactory: Service<crate::http::ConnInfo, Response = App>,
        AppFactory::Error: std::fmt::Debug + Send,
        AppFactory::Future: Send + 'static,
        App: Service<crate::http::Request, Response = crate::http::Response>,
        App: Send,
        App::Error: std::fmt::Debug,
        App::Future: Send + 'static,
    {
        let mut conn_number = 0;
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            conn_number += 1;
            let conn_info = crate::http::ConnInfo {
                host_and_port: format!("Fake info, connection #{}", conn_number),
            };

            let app = match app_factory.ready().await {
                Err(e) => {
                    eprintln!("Service not able to accept connections: {:?}", e);
                    continue;
                }
                Ok(app) => app,
            };
            let future = app.call(conn_info.clone());
            tokio::spawn(async move {
                match future.await {
                    Ok(app) => {
                        println!("Accepted a connection: {:?}", conn_info);
                        run_inner(app).await;
                    }
                    Err(e) => eprintln!("Error occurred: {:?}", e),
                }
            });
        }
    }

    async fn run_inner<App>(mut app: App)
    where
        App: Service<crate::http::Request, Response = crate::http::Response>,
        App::Error: std::fmt::Debug,
        App::Future: Send + 'static,
    {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

            let req = crate::http::Request {
                path_and_query: "/fake/path?page=1".to_owned(),
                headers: HashMap::new(),
                body: Vec::new(),
            };

            let app = match app.ready().await {
                Err(e) => {
                    eprintln!("Service not able to accept requests: {:?}", e);
                    continue;
                }
                Ok(app) => app,
            };
            let future = app.call(req);
            tokio::spawn(async move {
                match future.await {
                    Ok(res) => println!("Successful response: {:?}", res),
                    Err(e) => eprintln!("Error occurred: {:?}", e),
                }
            });
        }
    }
}

mod util {
    use std::{future::Future, task::Poll};

    pub struct AppFactoryFn<F> {
        f: F,
    }

    pub fn app_factory_fn<F, Ret, App>(f: F) -> AppFactoryFn<F>
    where
        F: FnMut(crate::http::ConnInfo) -> Ret,
        Ret: Future<Output = Result<App, anyhow::Error>>,
    {
        AppFactoryFn { f }
    }

    impl<F, Ret, App> tower::Service<crate::http::ConnInfo> for AppFactoryFn<F>
    where
        F: FnMut(crate::http::ConnInfo) -> Ret,
        Ret: Future<Output = Result<App, anyhow::Error>>,
    {
        type Response = App;
        type Error = anyhow::Error;
        type Future = Ret;

        fn poll_ready(
            &mut self,
            _cx: &mut std::task::Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(())) // always ready to accept a connection
        }

        fn call(&mut self, req: crate::http::ConnInfo) -> Self::Future {
            (self.f)(req)
        }
    }

    pub struct AppFn<F> {
        f: F,
    }

    pub fn app_fn<F, Ret>(f: F) -> AppFn<F>
    where
        F: FnMut(crate::http::Request) -> Ret,
        Ret: Future<Output = Result<crate::http::Response, anyhow::Error>>,
    {
        AppFn { f }
    }

    impl<F, Ret> tower::Service<crate::http::Request> for AppFn<F>
    where
        F: FnMut(crate::http::Request) -> Ret,
        Ret: Future<Output = Result<crate::http::Response, anyhow::Error>>,
    {
        type Response = crate::http::Response;
        type Error = anyhow::Error;
        type Future = Ret;

        fn poll_ready(
            &mut self,
            _cx: &mut std::task::Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(())) // always ready to accept a connection
        }

        fn call(&mut self, req: crate::http::Request) -> Self::Future {
            (self.f)(req)
        }
    }
}

#[tokio::main]
async fn main() {
    let counter = Arc::new(AtomicUsize::new(0));
    let mk_app = |_conn| {
        util::app_fn(move |mut req| {
            // need to clone this from the closure before moving it into the async block
            let counter = counter.clone();
            async move {
                println!("Handling a request for {}", req.path_and_query);
                let counter = counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                anyhow::ensure!(counter % 4 != 2, "Failing 25% of the time, just for fun");
                req.headers
                    .insert("X-Counter".to_owned(), counter.to_string());
                let res = crate::http::Response {
                    status: 200,
                    headers: req.headers,
                    body: req.body,
                };
                Ok::<_, anyhow::Error>(res)
            }
        })
    };
    let app_factory = util::app_factory_fn(|conn| {
        println!("Starting a new app for connection {:?}", conn);
        let app = (mk_app.clone())(conn);
        async move { Ok(app) }
    });
    fakeserver::run(app_factory).await;
}
