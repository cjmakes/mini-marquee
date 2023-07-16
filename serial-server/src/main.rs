use axum::{extract::Form,
 response::Html,
 routing::get,
 response::Redirect,
 Router
};
use serde::Deserialize;
use std::io::Write;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(show_form).post(accept_form));
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn show_form() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                <form action="/" method="post">
                    <label for="idata">
                        <input type="text" name="data" autofocus>
                    </label>
                    <input type="submit" value="show">
                </form>
            </body>
        </html>
        "#,
    )
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Input {
    data: String,
}

async fn accept_form(Form(input): Form<Input>) -> Redirect {
    let mut port = serialport::new("/dev/ttyACM0", 115_200)
        .open_native()
        .expect("Failed to open port");
    port.write(&[b'C']).expect("write failed!");
    std::thread::sleep(std::time::Duration::from_millis(10));
    port.write(input.data.as_bytes()).expect("write failed!");
    Redirect::to("/")
}
