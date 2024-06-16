use std::collections::HashMap;

use serde::Serialize;

use crate::models::History;

#[derive(Serialize, Debug)]
pub struct Breakdown(pub(crate) HashMap<String, [f32; 4]>);

impl Breakdown {
    pub fn from_base(value: History) -> Self {
        let mut mini_history = Self(HashMap::new());

        for hm_service in value.0.into_values() {
            for (date, entries) in hm_service {
                let minih_date = mini_history.0.entry(date).or_default();
                for s in entries {
                    minih_date[s as usize] += 1.0
                }
            }
        }

        for minih_date in mini_history.0.values_mut() {
            let total = minih_date[0] + minih_date[1] + minih_date[2] + minih_date[3];

            minih_date[0] = (minih_date[0] / total) * 100.0;
            minih_date[1] = (minih_date[1] / total) * 100.0;
            minih_date[2] = (minih_date[2] / total) * 100.0;
            minih_date[3] = (minih_date[3] / total) * 100.0;
        }

        mini_history
    }
}
