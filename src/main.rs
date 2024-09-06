#[macro_use]
extern crate rocket;
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use rocket::fs::{relative, NamedFile};
use rocket::http::CookieJar;
use rocket::http::Status;
use rocket::response::content::RawHtml;
use sha2::Sha256;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

const SECRET: &str = "21ae06b4b673da8996848710ecf3ef0c";

#[get("/")]
fn index(cookies: &CookieJar<'_>) -> Result<RawHtml<String>, Status> {
    let token = match signjwt() {
        Ok(token) => token,
        Err(_) => return Err(Status::InternalServerError),
    };
    cookies.add(("token", token));
    Ok(RawHtml(
        "Hello, world! 
        <br>Look at my picture!
    <br><a href='/view?name=gambar1.jpg'>gambar1</a>
    <br><a href='/view?name=gambar2.jpg'>gambar2</a>
    <br><a href='/view?name=gambar3.jpg'>gambar3</a>
    <br><a href='/view?name=gambar4.jpg'>gambar4</a>"
            .to_string(),
    ))
}

#[get("/view?<name>")]
async fn view(name: Option<String>) -> Result<NamedFile, Status> {
    match name {
        Some(name) => {
            if name.contains("flag") {
                Err(Status::Forbidden)
            } else {
                let path = Path::new(relative!("file_gambar")).join(name);
                let file = NamedFile::open(path).await;
                match file {
                    Ok(file) => Ok(file),
                    Err(_) => Err(Status::NotFound),
                }
            }
        }
        None => Err(Status::BadRequest),
    }
}

#[get("/c1edb8d965eac493eb139019afb4569caaa6e12167168b48380fa437dfa6cb2f69fae7d08fc13af6262d5ac137d4113246923eef5940bef74321e0ee2860343e")]
async fn flag(cookies: &CookieJar<'_>) -> Result<String, Status> {
    let admin = match cookies.get("token") {
        Some(token) => match verifyjwt(token.value()) {
            Ok(admin) => admin,
            Err(_) => false,
        },
        None => false,
    };
    if !admin {
        let info = "How do you find this page? Only admin can access this page.";
        return Ok(info.to_string());
    } else {
        let path = Path::new(relative!("5cdf9be3326a66461fbfc32482bd3cceec83e01c02cb2a5f4e2554151e8ed64ea233f7fa4e74babd1d39b874f4b353adc3f8aa9ac2e1c4d393be7dddfd756a90")).join("flag.txt");
        let file = fs::read_to_string(path);
        match file {
            Ok(file) => Ok(file),
            Err(_) => Err(Status::NotFound),
        }
    }
}

fn signjwt() -> Result<String, jwt::Error> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(SECRET.as_bytes())?;
    let mut claims = BTreeMap::new();
    claims.insert("admin".to_string(), "false".to_string());
    let token_str = match claims.sign_with_key(&key) {
        Ok(token) => token,
        Err(err) => return Err(jwt::Error::from(err)),
    };
    Ok(token_str)
}

fn verifyjwt(token: &str) -> Result<bool, jwt::Error> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(SECRET.as_bytes())?;
    let claims: BTreeMap<String, String> = match token.verify_with_key(&key) {
        Ok(claims) => claims,
        Err(err) => return Err(jwt::Error::from(err)),
    };
    match claims.get("admin") {
        Some(admin) => {
            if admin == "true" {
                Ok(true)
            } else {
                Ok(false)
            }
        }
        None => Ok(false),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![view])
        .mount("/", routes![flag])
}
