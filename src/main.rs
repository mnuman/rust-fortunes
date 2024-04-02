use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use psutil::memory;
use serde::Serialize;
use uptime_lib;
use actix_web_prom::PrometheusMetricsBuilder;
mod fortunes;



// Health endpoint JSON
#[derive(Serialize)]
struct HealthCheck {
    name: String,
    status: String,
}

#[derive(Serialize)]
struct HealthStatus {
    uptime: u64,
    memory_usage: f32,
    fortunes_served: u64,
    unique_fortunes: u64,
    checks: Vec<HealthCheck>,
}

async fn health() -> impl Responder {
    // implement a file-based health check
    // Return a 500 if a file exists
    if std::path::Path::new("error.txt").exists() {
        return HttpResponse::InternalServerError().finish();
    }
     
    let mut checks = vec![];

    // Check container uptime
    let uptime = uptime_lib::get().unwrap().as_secs();
    checks.push(HealthCheck {
        name: "Container uptime".to_string(),
        status: format!("{} seconds", uptime),
    });

    // Check memory usage
    let memory_usage = memory::virtual_memory().unwrap();
    let memory = memory_usage.percent();
    checks.push(HealthCheck {
        name: "Memory usage".to_string(),
        status: format!("{} %", memory.to_string()),
    });

    // Return the checks as a JSON object
    let health_status = HealthStatus {
        uptime,
        memory_usage: memory_usage.percent(),
        fortunes_served: fortunes::get_fortunes_served(),
        unique_fortunes: fortunes::unique_fortunes(),
        checks,
    };
    HttpResponse::Ok().json(health_status)
}

async fn fortune() -> impl Responder {
    HttpResponse::Ok().body(fortunes::get_random_fortune())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let prometheus = PrometheusMetricsBuilder::new("fortunes")
        .endpoint("/metrics")
        .build()
        .unwrap();

    HttpServer::new(move || {
        App::new()      
        .wrap(prometheus.clone())
        .service(web::resource("/fortune").route(web::get().to(fortune)))
        .service(web::resource("/health").route(web::get().to(health)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}