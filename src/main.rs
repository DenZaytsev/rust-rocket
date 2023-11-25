use rocket_prometheus::PrometheusMetrics;
#[macro_use]
extern crate rocket;

#[get("/<name>/<age>")]
fn wave(name: &str, age: u8) -> String {
    format!("👋 Hello, {} year old named {}!", age, name)
}

#[rocket::launch]
fn launch() -> _ {
    let prometheus = PrometheusMetrics::new();
    rocket::build()
        .attach(prometheus.clone())
        .mount("/metrics", prometheus)
        .mount("/wave", routes![wave])
}
