use crate::config::ServiceCode;
use crate::processors::ProcessorResult;

#[cfg(test)]

#[test]
fn result_display() {
    assert_eq!(format!("{}", ProcessorResult {
        status: ServiceCode::Online,
        ping: 123,
        host: "test.invalid".to_string(),
    }), "test.invalid is online with a ping of 123 ms")
}
