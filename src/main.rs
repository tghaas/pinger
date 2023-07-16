use std::time::Duration;
use ping_rs::*;
use influxdb::{Client, ReadQuery};
use influxdb::InfluxDbWriteable;
use chrono::{DateTime, Utc};
use tokio::time::sleep;

const PING_OPTS: PingOptions = PingOptions { ttl: 128, dont_fragment: true };
const TIMEOUT: Duration = Duration::from_secs(1);
const DATA: [u8; 24] = [255; 24];  // ping data
const SLEEP_TIME: u64 = 30;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let addr = "8.8.8.8".parse().unwrap();
    loop {
        let result = send_ping(&addr, TIMEOUT, &DATA, Some(&PING_OPTS));
        let cur_time = chrono::offset::Utc::now();
        match result {
            Ok(reply) => {
                println!("Reply from {}: bytes={} time={}ms TTL={}", reply.address, DATA.len(), reply.rtt, PING_OPTS.ttl);
                write_influx(reply.rtt, cur_time).await;
            }
            Err(e) => {
                println!("ERROR: {:?}", e)
            }
        }
        println!("Sleeping for {} seconds", SLEEP_TIME);
        sleep(Duration::from_secs(SLEEP_TIME)).await;
    }
}

async fn write_influx(response_time: u32, current_time: DateTime<Utc>){
    let client = Client::new("http://wellogger.h.local:8086", "ping");

    #[derive(InfluxDbWriteable)]
    struct PingResponse {
        time: DateTime<Utc>,
        ping_response: u32,
    }

    let ping_response = vec!(
        PingResponse {
            time: current_time,
            ping_response: response_time
        }.into_query("ping")
    );

    let write_result = client
        .query(ping_response)
        .await;
    assert!(write_result.is_ok(), "Write result was not okay {:?}", write_result.err());
    // Let's see if the data we wrote is there
    let read_query = ReadQuery::new("SELECT * FROM ping");

    let read_result = client.query(read_query).await;
    assert!(read_result.is_ok(), "Read result was not ok");
}
