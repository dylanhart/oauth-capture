extern crate clap;
extern crate iron;
extern crate urlencoded;

use clap::{Arg, App};

use iron::prelude::*;
use iron::status;
use urlencoded::UrlEncodedQuery;

fn main() {
    let args = App::new("OAuth Capture")
        .version("0.1.0")
        .author("Dylan Hart <dylan96hart@gmail.com>")
        .about("Captures oauth tokens")
        .arg(Arg::with_name("port")
            .short("p")
            .long("port")
            .value_name("PORT")
            .default_value("8080")
            .validator(validate_port))
        .arg(Arg::with_name("token param")
            .short("t")
            .long("token-param")
            .value_name("TOKEN_PARAM")
            .default_value("access_token"))
        .get_matches();

    let token_param = args.value_of("token param")
        .unwrap().to_string();

    let port = args.value_of("port").unwrap();
    let host = "localhost:".to_string() + port;

    println!("starting server at http://{}", host);
    Iron::new(move |req: &mut Request| {
        println!("request - {}", req.url.path().join("/"));
        println!("    query: {}", req.url.query().unwrap_or("<no query string>"));

        let token: String = req.get::<UrlEncodedQuery>().ok()
            .and_then(|mut map| {
                map.get_mut(&token_param)
                    .and_then(|e| e.pop())
            })
            .unwrap_or("<token not found>".to_string());

        println!("    token: {}", &token);

        Ok(Response::with((status::Ok, token)))
    }).http(&host as &str).expect("failed to start server");
}

fn validate_port(s: String) -> Result<(), String> {
    s.parse::<u16>()
        .map(|_| ())
        .map_err(|_| "Failed to parse port number".to_string())
}
