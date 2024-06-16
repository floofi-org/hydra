use std::fs;
use std::collections::HashMap;
use std::time::SystemTime;

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

use crate::error::StatusError;
use super::service::{Service, ServiceStatus};

type ServiceHistory = HashMap<String, Vec<ServiceStatus>>;

fn get_current_date() -> String {
    let now: DateTime<Utc> = SystemTime::now().into();
    now.format("%Y-%m-%d").to_string()
}

fn should_keep_entry(current_date: NaiveDate, entry_key: &str) -> bool {
    let date = NaiveDate::parse_from_str("%Y-%m-%d", entry_key)
        .unwrap_or_default();
    let difference = current_date - date;

    difference.num_days() <= 90
}

#[derive(Deserialize, Serialize)]
pub struct History(pub HashMap<String, ServiceHistory>);

impl History {
    fn get_id_for_service(&self, service: &Service) -> String {
        // First check for legacy key format
        if self.0.contains_key(service.get_legacy_id()) {
            service.get_legacy_id().to_owned()
        } else {
            service.get_unique_id()
        }
    }

    pub fn add_entry(&mut self, service: &Service, code: ServiceStatus) {
        let id = self.get_id_for_service(service);

        let service = self.0.entry(id).or_default();
        let date = get_current_date();

        let statuses = service.entry(date).or_default();
        statuses.push(code);
    }

    pub fn vacuum(&mut self) {
        let now: DateTime<Utc> = SystemTime::now().into();
        let now = now.date_naive();

        self.0.values_mut()
            .for_each(|service| service.retain(|date, _| should_keep_entry(now, date)));
    }

    pub fn sync(&self) -> Result<(), StatusError> {
        let text = serde_json::to_string(&self)?;
        fs::write("./history.json", text)?;
        Ok(())
    }
}
