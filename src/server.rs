use actix::{
    self,
    prelude::*,
};
use actix_files as fs;
use actix_web::{
    middleware, 
    App, 
    HttpServer,
    http::ContentEncoding,
};
use failure::Fallible;

use crate::URL;

pub struct Server {
    runner: SystemRunner,
}

impl Server {
    pub fn new() -> Fallible<Self> {
        // Build a new actor system
        let runner = actix::System::new("backend");
        let server = HttpServer::new(move || {
            let builder = App::new()
                .wrap(middleware::Compress::new(ContentEncoding::Auto))
                .wrap(middleware::Logger::default())
                .wrap(middleware::DefaultHeaders::new()
                    .header("Cache-Control", "no-cache")
                    .header("Vary", "Accept-Encoding")
                );
                
                //Static files
                builder.service(fs::Files::new("/", "static").index_file("index.html"))
            });

        server.bind(URL)?.start();

        Ok(Server {
            runner,
        })
    }

    pub fn start(self) -> Fallible<()> {
        Ok(self.runner.run()?)
    }
}