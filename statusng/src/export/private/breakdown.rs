use std::collections::BTreeMap;
use chrono::NaiveDate;
use serde::Serialize;

use crate::models::service::ServiceStatus;
use crate::models::History;

type StatusBreakdown = [f32; 4];

#[derive(Serialize, Debug)]
pub struct Breakdown(pub(crate) BTreeMap<NaiveDate, StatusBreakdown>);

impl Breakdown {
    pub fn from_base(history: History) -> Self {
        let mut map: BTreeMap<NaiveDate, StatusBreakdown> = BTreeMap::new();

        for service in history.0.into_values() {
            for (date, statuses) in service {
                accumulate_entry(&mut map, date, &statuses);
            }
        }

        calc_percentages(&mut map);
        Self(map)
    }
}

fn accumulate_entry(
    map: &mut BTreeMap<NaiveDate, StatusBreakdown>,
    date: NaiveDate,
    statuses: &[ServiceStatus],
) {
    let entry = map.entry(date).or_default();
    for status in statuses {
        entry[*status as usize] += 1.0;
    }
}

fn calc_percentages(map: &mut BTreeMap<NaiveDate, StatusBreakdown>) {
    for (_, entry) in map.iter_mut() {
        let total: f32 = entry.iter().sum();
        entry.iter_mut().for_each(|s| *s = *s / total * 100.0);
    }
}
