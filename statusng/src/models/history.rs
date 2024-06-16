use std::collections::HashMap;
use std::fs;
use serde::{Deserialize, Serialize};
use crate::models::service::{Service, ServiceStatus};
use crate::error::StatusError;
use chrono::{DateTime, Utc};
use std::time::SystemTime;

type ServiceHistory = HashMap<String, Vec<ServiceStatus>>;

fn get_date() -> String {
    let now = SystemTime::now();
    let now: DateTime<Utc> = now.into();
    let now = now.to_rfc3339();
    now.split('T')
        .next().unwrap_or("0000-00-00")
        .to_string()
}

#[derive(Deserialize, Serialize)]
pub struct History(pub HashMap<String, ServiceHistory>);

impl History {
    pub fn add_entry(&mut self, service: &Service, code: ServiceStatus) {
        let hm_root = &mut self.0;

        let key = if hm_root.contains_key(service.get_legacy_id()) {
            service.get_legacy_id()
        } else {
            &service.get_unique_id()
        };

        let hm_service = hm_root.entry(key.to_string()).or_default();
        let date = get_date();

        let statuses = hm_service.entry(date).or_default();
        statuses.push(code);
    }

    pub fn vacuum(&mut self) {
        let now = SystemTime::now();
        let now: DateTime<Utc> = now.into();
        let now = now.date_naive();

        for hm_service in &mut self.0.values_mut() {
            hm_service.retain(|k, _| {
                let date_time = DateTime::parse_from_rfc3339(&format!("{}T00:00:00Z", k))
                    .unwrap_or_default().date_naive();
                let difference = now - date_time;

                difference.num_days() <= 90
            });
        }
    }

    pub fn sync(&self) -> Result<(), StatusError> {
        let text = serde_json::to_string(&self)?;
        fs::write("./history.json", text)?;
        Ok(())
    }
}
