use chrono::Utc;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct OpenC2Args {
    start_time: Option<u64>,
    stop_time: Option<u64>,
    response_requested: ResponseRequested,
}

impl OpenC2Args {
    pub fn new(
        start_time: Option<u64>,
        stop_time: Option<u64>,
        response_requested: ResponseRequested,
    ) -> Self {
        OpenC2Args {
            start_time,
            stop_time,
            response_requested,
        }
    }

    pub fn get_timeout(&self) -> Option<u64> {
        self.stop_time.map(|time| {
            let now = Utc::now().timestamp_millis();
            std::cmp::max(time as i64 - now, 0) as u64
        })
    }

    pub fn get_response_requested(&self) -> &ResponseRequested {
        &self.response_requested
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum ResponseRequested {
    None,
    Ack,
    Status,
    Complete,
}

#[test]
fn test_args_timeout() {
    let now = Utc::now().timestamp_millis() + 1_000;
    let args = OpenC2Args::new(None, Some(now as u64), ResponseRequested::None);
    assert!(args.get_timeout().unwrap() > 0);
}
