extern crate futures;
extern crate hyper;
extern crate tokio_core;

//use futures::Future;
use hyper::{Client, Uri, Response, Body, StatusCode};
use tokio_core::reactor::Core;

use std::future::Future as stdFuture;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("Hello, world!");
    let mut f = foo();
    loop {
        let poll = f.poll().unwrap();
        if poll.is_ready() {
            break;
        }
    }
    println!("finish");
}


async fn foo() -> bool {
    println!("start");
    let mut core = Core::new().unwrap();

    let client = Client::new();

    let url: Uri = "http://httpbin.org/response-headers?foo=bar"
        .parse().unwrap();
    assert_eq!(url.query(), Some("foo=bar"));

    let request_result = core.run(client
        .get(url)
        .map(|res: Response<Body>| {
            println!("Response: {}", res.status());
            res.status()
        })
        .map_err(|err| {
            println!("Error: {}", err);
            err
        })
    );

    return request_result.unwrap().is_success();
}

#[test]
fn test_http() {
    println!("start");
    let mut core = Core::new().unwrap();

    let client = Client::new();

    let url: Uri = "http://httpbin.org/response-headers?foo=bar".parse().unwrap();
    assert_eq!(url.query(), Some("foo=bar"));

    let request_result = core.run(client
        .get(url)
        .map(|res| {
            println!("Response: {}", res.status());
        })
        .map_err(|err| {
            println!("Error: {}", err);
        })
    );

    println!("end");
}