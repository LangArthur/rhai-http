use rhai::packages::Package;
use rhai_http::HttpPackage;
use rouille::{Response, Server};

fn start_server(
    addr: &str,
    response: String,
) -> (std::thread::JoinHandle<()>, std::sync::mpsc::Sender<()>) {
    let server = Server::new(addr, move |_request| {
        Response::text(response.clone()).with_status_code(200)
    })
    .unwrap();
    server.stoppable()
}

fn stop_server(handle: std::thread::JoinHandle<()>, sender: std::sync::mpsc::Sender<()>) {
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_secs(3));
        sender.send(()).unwrap();
    });
    handle.join().unwrap();
}

#[test]
fn simple_test() {
    let (handle, sender) = start_server("0.0.0.0:8080", "hello world".to_string());
    let mut engine = rhai::Engine::new();

    HttpPackage::new().register_into_engine(&mut engine);

    let body: String = engine
        .eval(
            r#"
let client = http::client();

client.request(#{ method: "GET", url: "http://0.0.0.0:8080" })"#,
        )
        .unwrap();
    assert_eq!(body, "hello world");
    stop_server(handle, sender);
}

#[test]
fn array_test() {
    let content = "[#{user: \"admin\"}, #{user: \"user\"}]".to_string();
    // FIXME: I think it does not return a json but a string
    let (handle, sender) = start_server("0.0.0.0:8081", content.clone());
    let mut engine = rhai::Engine::new();

    HttpPackage::new().register_into_engine(&mut engine);

    let body: String = engine
        .eval(
            r#"
let client = http::client();

client.request(#{ method: "GET", url: "http://0.0.0.0:8081" })"#,
        )
        .unwrap();
    assert_eq!(body, content);
    stop_server(handle, sender);
}
