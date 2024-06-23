use std::collections::HashMap;

use serde::Serialize;

use crate::models::service::ServiceStatus;
use crate::models::History;

type StatusBreakdown = [f32; 4];

#[derive(Serialize, Debug)]
pub struct Breakdown(pub(crate) HashMap<String, StatusBreakdown>);

impl Breakdown {
    pub fn from_base(history: History) -> Self {
        let mut map: HashMap<String, StatusBreakdown> = HashMap::new();

        for (date, statuses) in history.0.into_values().flatten() {
            accumulate_entry(&mut map, date, &statuses);
        }

        calc_percentages(&mut map);
        Self(map)
    }
}

fn accumulate_entry(
    map: &mut HashMap<String, StatusBreakdown>,
    date: String,
    statuses: &[ServiceStatus],
) {
    let entry = map.entry(date).or_default();
    for status in statuses {
        entry[*status as usize] += 1.0;
    }
}

fn calc_percentages(map: &mut HashMap<String, StatusBreakdown>) {
    let total = map.len() as f32;

    for (_, entry) in map.iter_mut() {
        entry.iter_mut().for_each(|s| *s = *s / total * 100.0);
    }
}
