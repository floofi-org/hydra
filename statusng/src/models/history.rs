use std::fs;
use std::collections::HashMap;
use std::time::SystemTime;

use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::error::{HistoryError, StatusError};
use super::service::{Service, ServiceStatus};

type ServiceHistory = HashMap<NaiveDate, Vec<ServiceStatus>>;

fn get_current_date() -> NaiveDate {
    let now: DateTime<Utc> = SystemTime::now().into();
    let now: NaiveDate = now.date_naive();
    now
}

fn should_keep_entry(current_date: NaiveDate, entry_key: &NaiveDate) -> bool {
    let difference = current_date - *entry_key;
    difference.num_days() <= 90
}

#[derive(Deserialize, Serialize, Debug, Clone)]
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
        let bytes = self.clone().into_bytes();
        fs::write("./history.dat", bytes)?;
        Ok(())
    }

    pub fn into_bytes(self) -> Vec<u8> {
        let mut bytes = vec![self.0.len() as u8];

        for (service, history) in self.0 {
            let mut service_bytes = service.into_bytes();

            bytes.push(service_bytes.len() as u8);
            bytes.append(&mut service_bytes);
            bytes.push(history.len() as u8);

            for (date, entries) in history {
                let date = NaiveDateTime::from(date);
                let date: DateTime<Utc> = DateTime::from_naive_utc_and_offset(date, Utc);
                let ts = date.timestamp() / 86400;

                bytes.append(&mut (ts as u32).to_le_bytes().to_vec());
                bytes.append(&mut (entries.len() as u16).to_le_bytes().to_vec());
                bytes.append(&mut (entries.iter().map(|i| *i as u8).collect()))
            }
        }

        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, HistoryError> {
        let mut history = HashMap::new();
        let mut iterator = bytes.iter();

        let services_count = *iterator.next().ok_or(HistoryError::EndOfFile)?;
        for _ in 0u8..services_count {
            let id_length = *iterator.next().ok_or(HistoryError::EndOfFile)?;
            let mut id_bytes = vec![];

            for _ in 0u8..id_length {
                id_bytes.push(*iterator.next().ok_or(HistoryError::EndOfFile)?);
            }

            let id = String::from_utf8(id_bytes)?;
            let entry: &mut ServiceHistory = history.entry(id).or_default();
            let days = *iterator.next().ok_or(HistoryError::EndOfFile)?;

            for _ in 0u8..days {
                let mut day_bytes = [0u8; 4];
                for i in 0..4 {
                    day_bytes[i] = *iterator.next().ok_or(HistoryError::EndOfFile)?;
                }
                let day = u32::from_le_bytes(day_bytes);
                let day = day as i64 * 86400;
                let day = DateTime::from_timestamp(day, 0).ok_or(HistoryError::InvalidDate)?;
                let day = day.date_naive();

                let entry = entry.entry(day).or_default();
                let mut entries_bytes = [0u8; 2];
                for i in 0..2 {
                    entries_bytes[i] = *iterator.next().ok_or(HistoryError::EndOfFile)?;
                }
                let entries = u16::from_le_bytes(entries_bytes);

                for _ in 0..entries {
                    entry.push(ServiceStatus::from(*iterator.next().ok_or(HistoryError::EndOfFile)?));
                }
            }
        }

        Ok(History(history))
    }
}
