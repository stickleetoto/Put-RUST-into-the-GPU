//! Simple JSON records for baseline experiment metrics.
use std::collections::BTreeMap;

#[derive(Debug, Default)]
pub struct ExperimentRecord {
    pub name: String,
    pub metrics: BTreeMap<String, f64>,
}

impl ExperimentRecord {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            metrics: BTreeMap::new(),
        }
    }
    pub fn add_metric(&mut self, name: impl Into<String>, value: f64) {
        self.metrics.insert(name.into(), value);
    }
    pub fn to_json(&self) -> String {
        serde_json::json!({ "name": &self.name, "metrics": &self.metrics }).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn serializes_json() {
        let mut run = ExperimentRecord::new("baseline");
        run.add_metric("loss", 0.5);
        let value: serde_json::Value = serde_json::from_str(&run.to_json()).unwrap();
        assert_eq!(value["metrics"]["loss"], 0.5);
    }
}
