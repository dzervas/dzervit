use actix_files as fs;
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize)]
struct APIRequest {
	target: Option<String>,
	item: Option<String>,
}

#[derive(Deserialize, Serialize)]
struct APIv1Res {
	success: bool,
	text: String,
}

#[get("/api/v1")]
async fn v1_api(info: web::Query<APIRequest>) -> impl Responder {
	let target = info.target.clone().unwrap_or_else(|| "You".into());
	let item = info.item.clone().unwrap_or_else(|| "it".into());
	let text = format!("{} deserve {}!", target, item);

	let res = APIv1Res {
		success: true,
		text,
	};

	HttpResponse::Ok().json(res)
}

async fn not_found(req: HttpRequest) -> impl Responder {
	let path = req.uri().path();
	let item = path.chars().next().map(|c| &path[c.len_utf8()..]);

	let final_path = match item {
		Some(i) => format!("/#{}", i),
		None => "/".to_string(),
	};

	HttpResponse::MovedPermanently()
		.append_header(("Location", final_path))
		.finish()
}

#[actix_web::main]
#[cfg(not(tarpaulin_include))]
async fn main() -> std::io::Result<()> {
    use actix_web::middleware;

	HttpServer::new(|| {
		App::new()
			.wrap(middleware::Logger::default())
			.wrap(middleware::NormalizePath::trim())
			.service(v1_api)
			.service(fs::Files::new("/", concat!(env!("CARGO_MANIFEST_DIR"), "/bundle")).index_file("index.html"))
			.default_service(web::route().to(not_found))
	})
	.bind(("127.0.0.1", 8000))?
	.run()
	.await
}

#[cfg(test)]
mod tests {
	extern crate serde_json;

	use super::*;
	use actix_test::TestRequest;
	use actix_web::http::header;
	use actix_web::middleware;
	use actix_web::test;

	// #[actix_rt::test]
	// async fn v1_api_test() {
	// 	let param_list: Vec<Vec<Option<String>>> = vec![
	// 		vec![None,				None,					Some("You deserve it!".into())],
	// 		vec![Some("I".into()),  None,					Some("I deserve it!".into())],
	// 		vec![None,				Some("this".into()),	Some("You deserve this!".into())],
	// 		vec![Some("I".into()),  Some("this".into()),	Some("I deserve this!".into())],
	// 	];

	// 	for param in param_list {
	// 		let target = param.get(0).unwrap();
	// 		let item = param.get(1).unwrap();
	// 		let expected = param.get(2).unwrap();

	// 		let res = v1_api(Some(APIRequest { target: *target, item: *item })).await;
	// 		let res_json = res.into_inner();

	// 		assert!(res_json.success, "{:?} parameters resulted in unsuccessful responses", &param);
	// 		assert_eq!(res_json.text, expected.unwrap().to_string());
	// 	}
	// }

	#[actix_rt::test]
	async fn v1_api_integration_test() {
		let mut app = test::init_service(
			App::new()
				.wrap(middleware::NormalizePath::trim())
				.service(v1_api)
			).await;


		let req_res_list: Vec<Vec<&str>> = vec![
			vec!["/api/v1",									"You deserve it!"],
			vec!["/api/v1/",								"You deserve it!"],
			vec!["/api/v1/?target=I",						"I deserve it!"],
			vec!["/api/v1/?target=I&abc=asdf",				"I deserve it!"],
			vec!["/api/v1/?asrf=aaa&target=I",				"I deserve it!"],
			vec!["/api/v1/?abc=asdf&target=I&",				"I deserve it!"],
			vec!["/api/v1/?abc=asdf&target=I&555123=1234",	"I deserve it!"],

			vec!["/api/v1/?item=this",						"You deserve this!"],
			vec!["/api/v1/?item=this&abc=asdf",				"You deserve this!"],
			vec!["/api/v1/?asrf=aaa&item=this",				"You deserve this!"],
			vec!["/api/v1/?abc=asdf&item=this&",			"You deserve this!"],
			vec!["/api/v1/?abc=asdf&item=this&555123=1234", "You deserve this!"],

			vec!["/api/v1/?target=He&item=1234",			"He deserve 1234!"],
			vec!["/api/v1/?item=1234&target=He",			"He deserve 1234!"],
			vec!["/api/v1/?item=1234&hell=world&target=He", "He deserve 1234!"],

			vec!["/api/v1/?item=the%20gift&target=She&&&&",	"She deserve the gift!"],
		];

		for req_res_item in req_res_list {
			let url = req_res_item.get(0).unwrap();
			let expected = req_res_item.get(1).unwrap();

			let res = TestRequest::get()
				.uri(url)
				.append_header((header::CONTENT_TYPE, "application/json"))
				.send_request(&mut app).await;

			eprintln!("{:?}", req_res_item);
			eprintln!("{:?}", res);
			assert!(res.status().is_success());

			let res_json: APIv1Res = test::read_body_json(res).await;

			assert!(res_json.success);
			assert_eq!(res_json.text, expected.to_string());
		}
	}
}
