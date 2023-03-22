mod test_1;
mod update_event_task;

use std::sync::Mutex;

use actix_web::{web, App, HttpResponse, HttpServer, Responder, Result};
use actix_web::guard::Guard;
use prometheus_client::encoding::text::{encode, Encode};
use prometheus_client::metrics::counter::Counter;
use prometheus_client::metrics::gauge::Gauge;
use prometheus_client::metrics::family::Family;
use prometheus_client::registry::Registry;
use storage::MysqlStorage;
use storage::operator::Status;

use env_logger;
use tracing::{debug, error, info, warn};


#[derive(Clone, Hash, PartialEq, Eq, Encode)]
pub enum Method {
    Get,
    Post,
}

#[derive(Clone, Hash, PartialEq, Eq, Encode)]
pub struct MethodLabels {
    pub method: Method,
}

#[derive(Clone, Hash, PartialEq, Eq, Encode)]
pub struct BalanceLabels {
    pub address: String
}



#[derive(Clone, Hash, PartialEq, Eq, Encode)]
pub struct AccountLabels {
    pub address: String
}


#[derive(Clone, Hash, PartialEq, Eq, Encode)]
pub struct OperatorLabels {
    pub id: u32
}

pub struct Metrics {
    account_balance: Family<AccountLabels, Gauge>,
    operator_performance: Family<OperatorLabels, Gauge>,
    operator_status: Family<OperatorLabels, Gauge>,
    operator_fee: Family<OperatorLabels, Gauge>,
}

impl Metrics {
    pub fn inc_requests(&self, method: Method) {
        // self.requests.get_or_create(&MethodLabels { method }).inc();
        // self.requests_2.get_or_create(&BalanceLabels{address: "0x0001".to_string()}).set(33);
        // self.requests_2.get_or_create(&BalanceLabels{address: "0x0002".to_string()}).set(44);
        // self.requests_2.get_or_create(&BalanceLabels{address: "0x0003".to_string()}).set(55);
    }

    pub fn set_operator_performance(&self, operator_id: u32, performance: f32) {
        let performance_u64 = performance as u64;
        let operator_labels = OperatorLabels{id: operator_id};
        self.operator_performance.get_or_create(&operator_labels).set(performance_u64);
    }

    pub fn set_account_balance(&self, address: String, balance: f32) {
        let balance_u64 = balance as u64;
        self.account_balance.get_or_create(&AccountLabels{address}).set(balance_u64);
    }


    pub fn set_operator_status(&self, operator_id: u32, status: u32) {
        let status_u64 = status as u64;
        let operator_labels = OperatorLabels{id: operator_id};
        self.operator_status.get_or_create(&operator_labels).set(status_u64);
    }

    pub fn set_operator_fee(&self, operator_id: u32, fee: f32) {
        let fee_u64 = fee as u64;
        let operator_labels = OperatorLabels{id: operator_id};
        self.operator_performance.get_or_create(&operator_labels).set(fee_u64);
    }
}

pub struct AppState {
    pub registry: Registry,
}

pub async fn metrics_handler(state: web::Data<Mutex<AppState>>) -> Result<HttpResponse> {
    let state = state.lock().unwrap();
    let mut buf = Vec::new();
    encode(&mut buf, &state.registry)?;
    let body = std::str::from_utf8(buf.as_slice()).unwrap().to_string();
    Ok(HttpResponse::Ok()
        .content_type("application/openmetrics-text; version=1.0.0; charset=utf-8")
        .body(body))
}

pub async fn some_handler(metrics: web::Data<Metrics>) -> impl Responder {
    metrics.inc_requests(Method::Get);
    "okay".to_string()
}

pub fn update_operator_performance() {

}


pub fn update(metrics: web::Data<Metrics>, storage_connect_url: String) {
    let mysql_storage = MysqlStorage::new(storage_connect_url);
    let ops = mysql_storage.update_operator(
        127,
        "0x000".to_string(),
        "0x0001".to_string(),
        Status::Removed,
        444,
        1.1,
        1.2,
        23.3
    );
    println!("la")
    // let operators =  storage.get_all_operators();

}


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let metrics = web::Data::new(Metrics {
        account_balance: Family::default(),
        operator_performance: Family::default(),
        operator_status: Family::default(),
        operator_fee: Family::default(),
    });
    let mut state = AppState {
        registry: Registry::default(),
    };

    state.registry.register(
        "account_balance",
        "balance of account",
        Box::new(metrics.account_balance.clone()),
    );

    let state = web::Data::new(Mutex::new(state));

    println!("starting");

    let connect_url = "mysql://root:wonders,1@192.168.1.128:3308/ssv_alert_service_5".to_string();

    update(metrics.clone(), connect_url);

    HttpServer::new(move || {
        App::new()
            .app_data(metrics.clone())
            .app_data(state.clone())
            .service(web::resource("/metrics").route(web::get().to(metrics_handler)))
            .service(web::resource("/handler").route(web::get().to(some_handler)))
    })
        .bind(("0.0.0.0", 9016))?
        .run()
        .await
}

pub fn test_log_log() {
    error!("test_logtest_logtest_logtest_log")
}


#[cfg(test)]
mod test {

    use env_logger;
    use tracing::{debug, error, info, warn};
    use crate::test_log_log;

    fn init() {
        // env_logger::init()
        env_logger::builder().is_test(true).init();
        error!("inited error log");
        println!("inited")
    }

    #[test]
    fn test_log() {
        env_logger::builder()
            .is_test(true).init();
        error!("aaaaa bbb");
        test_log_log();
        println!("ended")
    }
}