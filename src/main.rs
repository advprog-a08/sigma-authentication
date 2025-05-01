use sigma_authentication::app::App;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let app = App::default();
    app.launch().await
}
