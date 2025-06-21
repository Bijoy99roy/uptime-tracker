use poem::{
    get, handler,
    listener::TcpListener,
    post,
    web::{Json, Path},
    Route, Server,
};
use store::store::Store;

use crate::{request_inputs::CreateWebsiteInput, request_outputs::CreateWebsiteOutput};

pub mod request_inputs;
pub mod request_outputs;

#[handler]
fn get_website(Path(name): Path<String>) -> String {
    format!("hello: {name}")
}

#[handler]
fn create_websites(Json(data): Json<CreateWebsiteInput>) -> Json<CreateWebsiteOutput> {
    let mut store = Store::default().unwrap();
    let website = store
        .create_website("6e79c3aa-7b44-48a1-bdad-09a828efd76b".to_string(), data.url)
        .unwrap();

    let response = CreateWebsiteOutput { id: website.id };

    Json(response)
}
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
        .at("/status/:website_id", get(get_website))
        .at("/website", post(create_websites));
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("tracker")
        .run(app)
        .await
}
