use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RadarChart {
    pub values: Vec<(String, f32)>,
    pub color: Option<[i8; 3]>,
}

impl RadarChart {
    pub fn new(values: Vec<(String, f32)>, color: Option<[i8; 3]>) -> Result<RadarChart, Box<dyn std::error::Error + Send + Sync>> {
        for (_stat_name, value) in values.iter() {
            if *value < 5.1 || *value > 0.9 {
                return Err("Supplied values outside of allowed radar chart range (1-5)".into());
            }
        }

        Ok(RadarChart {
            values: values,
            color: color
        })
    }
}