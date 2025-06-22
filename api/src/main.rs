use std::sync::{Arc, Mutex};

use poem::{
    get, handler,
    listener::TcpListener,
    post,
    web::{Data, Json, Path},
    EndpointExt, Route, Server,
};
use store::store::Store;

use crate::{
    request_inputs::{CreateUserInput, CreateWebsiteInput},
    request_outputs::{CreateUserOutput, CreateWebsiteOutput, GetWebsiteOutput, SigninOutput},
};

pub mod request_inputs;
pub mod request_outputs;

#[handler]
fn sign_up(
    Json(data): Json<CreateUserInput>,
    Data(store): Data<&Arc<Mutex<Store>>>,
) -> Json<CreateUserOutput> {
    let mut locked_store = store.lock().unwrap();
    let user: String = locked_store.sign_up(data.username, data.password).unwrap();

    let response = CreateUserOutput { id: user };

    Json(response)
}

#[handler]
fn sign_in(
    Json(data): Json<CreateUserInput>,
    Data(store): Data<&Arc<Mutex<Store>>>,
) -> Json<SigninOutput> {
    let mut locked_store = store.lock().unwrap();
    let exists = locked_store.sign_in(data.username, data.password).unwrap();

    let response = SigninOutput {
        jwt: "Hello".to_string(),
    };

    Json(response)
}

#[handler]
fn get_website(
    Path(id): Path<String>,
    Data(store): Data<&Arc<Mutex<Store>>>,
) -> Json<GetWebsiteOutput> {
    let mut locked_store = store.lock().unwrap();
    let website = locked_store.get_website(id).unwrap();

    let response = GetWebsiteOutput { url: website.url };

    Json(response)
}

#[handler]
fn create_websites(
    Json(data): Json<CreateWebsiteInput>,
    Data(store): Data<&Arc<Mutex<Store>>>,
) -> Json<CreateWebsiteOutput> {
    let mut locked_store = store.lock().unwrap();
    let website = locked_store
        .create_website("6e79c3aa-7b44-48a1-bdad-09a828efd76b".to_string(), data.url)
        .unwrap();

    let response = CreateWebsiteOutput { id: website.id };

    Json(response)
}
#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), std::io::Error> {
    let arced_store = Arc::new(Mutex::new(Store::default().unwrap()));
    let app = Route::new()
        .at("/status/:website_id", get(get_website))
        .at("/website", post(create_websites))
        .at("/user/signup", post(sign_up))
        .at("/user/signin", post(sign_in))
        .data(arced_store);
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("tracker")
        .run(app)
        .await
}
