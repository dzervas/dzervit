#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::{Request, Rocket, http::{RawStr, hyper::header::Location}};
use rocket_contrib::json::Json;
use rocket_contrib::serve::StaticFiles;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct APIv1Res {
    success: bool,
    text: String,
}

#[get("/?<target>&<item>")]
fn v1_api(target: Option<&RawStr>, item: Option<&RawStr>) -> Json<APIv1Res> {
    let text = format!("{} deserve {}!", target.unwrap_or("You".into()), item.unwrap_or("it".into()));
    Json(APIv1Res {
        success: true,
        text: text,
    })
}
#[derive(Responder)]
#[response(status=301)]
struct RawRedirect((), Location);

#[catch(404)]
fn index_item(req: &Request) -> RawRedirect {
    let path = req.uri().path();
    let item = path.chars().next().map(|c| &path[c.len_utf8()..]);

    let final_path = match item {
        Some(i) => format!("/#{}", i),
        None => "/".to_string(),
    };

    println!("Redirecting to {:?}", final_path);

    RawRedirect((), Location(final_path))
}

fn rocket() -> Rocket {
    rocket::ignite()
        .mount("/api/v1/", routes![v1_api])
        .mount("/", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/bundle")))
        .register(catchers![index_item])
}

#[cfg(not(tarpaulin_include))]
fn main() {
    rocket().launch();
}

#[cfg(test)]
mod tests {
    extern crate serde_json;

    use super::*;
    use rocket::{http::{ContentType, Status}, local::Client};

    #[test]
    fn v1_api_test() {
        let param_list: Vec<Vec<Option<&RawStr>>> = vec![
            vec![None,              None,                   Some("You deserve it!".into())],
            vec![Some("I".into()),  None,                   Some("I deserve it!".into())],
            vec![None,              Some("this".into()),    Some("You deserve this!".into())],
            vec![Some("I".into()),  Some("this".into()),    Some("I deserve this!".into())],
        ];

        for param in param_list {
            let target = param.get(0).unwrap();
            let item = param.get(1).unwrap();
            let expected = param.get(2).unwrap();

            let res = v1_api(*target, *item);
            let res_json = res.into_inner();

            assert!(res_json.success, format!("{:?} parameters resulted in unsuccessful responses", &param));
            assert_eq!(res_json.text, expected.unwrap().to_string());
        }
    }

    #[test]
    fn v1_api_integration_test() {
        let client = Client::new(rocket()).expect("Valid rocket instance");

        let req_res_list: Vec<Vec<&str>> = vec![
            vec!["/api/v1/",                                "You deserve it!"],
            vec!["/api/v1/?target=I",                       "I deserve it!"],
			vec!["/api/v1/?target=I&abc=asdf",              "I deserve it!"],
            vec!["/api/v1/?asrf=aaa&target=I",              "I deserve it!"],
            vec!["/api/v1/?abc=asdf&target=I&",             "I deserve it!"],
            vec!["/api/v1/?abc=asdf&target=I&555123=1234",  "I deserve it!"],

            vec!["/api/v1/?item=this",                      "You deserve this!"],
			vec!["/api/v1/?item=this&abc=asdf",             "You deserve this!"],
            vec!["/api/v1/?asrf=aaa&item=this",             "You deserve this!"],
            vec!["/api/v1/?abc=asdf&item=this&",            "You deserve this!"],
            vec!["/api/v1/?abc=asdf&item=this&555123=1234", "You deserve this!"],

            vec!["/api/v1/?target=He&item=1234",            "He deserve 1234!"],
            vec!["/api/v1/?item=1234&target=He",            "He deserve 1234!"],
            vec!["/api/v1/?item=1234&hell=world&target=He", "He deserve 1234!"],

            // vec!["/api/v1/?item=the%20gift&target=She&&&&",   "She deserve the gift!"],
        ];

        for req_res_item in req_res_list {
            let url = req_res_item.get(0).unwrap();
            let expected = req_res_item.get(1).unwrap();

            let mut res = client.get(*url).dispatch();
            let res_json: APIv1Res = serde_json::from_slice(&res.body_bytes().unwrap()).unwrap();

            assert_eq!(res.status(), Status::Ok);
            assert_eq!(res.content_type().unwrap(), ContentType::JSON);
            assert!(res_json.success);
            assert_eq!(res_json.text, expected.to_string());
        }
    }
}
