use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct Airtable {
    token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct AirtableResponse {
    records: Vec<AirtableRecord>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AirtableRecord {
    fields: HashMap<String, AirtableValue>,
    id: String,
}

impl AirtableRecord {
    pub fn string(&self, key: &str) -> Option<String> {
        self.fields.get(key).and_then(|value| match value {
            AirtableValue::String(s) => Some(s.to_string()),
            _ => None,
        })
    }

    pub fn float(&self, key: &str) -> Option<f64> {
        self.fields.get(key).and_then(|value| match value {
            AirtableValue::Number(n) => Some(*n),
            _ => None,
        })
    }

    pub fn integer(&self, key: &str) -> Option<i64> {
        self.fields.get(key).and_then(|value| match value {
            AirtableValue::Number(n) => Some(*n as i64),
            _ => None,
        })
    }

    pub fn records(&self, key: &str) -> Vec<String> {
        self.fields
            .get(key)
            .and_then(|value| match value {
                AirtableValue::Records(a) => Some(a.clone()),
                _ => Some(vec![]),
            })
            .unwrap_or(vec![])
    }

    pub fn boolean(&self, key: &str) -> Option<bool> {
        self.fields.get(key).and_then(|value| match value {
            AirtableValue::Boolean(b) => Some(*b),
            _ => None,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
enum AirtableValue {
    String(String),
    Number(f64),
    Records(Vec<String>),
    Boolean(bool),
}

impl Airtable {
    pub fn new(token: &str) -> Self {
        Self {
            token: token.to_string(),
        }
    }

    pub async fn records(self, base_id: &str, table_id: &str) -> Vec<AirtableRecord> {
        let client = reqwest::Client::new();
        let res = client
            .get(format!(
                "https://api.airtable.com/v0/{}/{}",
                base_id, table_id
            ))
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await
            .unwrap();

        res.json::<AirtableResponse>().await.unwrap().records
    }

    pub async fn record(
        self,
        base_id: &str,
        table_id: &str,
        record_id: &str,
    ) -> Option<AirtableRecord> {
        let client = reqwest::Client::new();
        let res = client
            .get(format!(
                "https://api.airtable.com/v0/{}/{}/{}",
                base_id, table_id, record_id
            ))
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await
            .unwrap();

        res.json::<AirtableRecord>().await.ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_records() {
        let records = create_airtable()
            .records("appxauMzM76PEp2Aw", "Issues")
            .await;

        println!("{:?}", records);
        assert_eq!(records.len(), 1);
    }

    #[tokio::test]
    async fn test_get_record() {
        let record = create_airtable()
            .record("appxauMzM76PEp2Aw", "Articles", "recppDpYVsvu2DfmU")
            .await;

        println!("{:?}", record);
    }

    fn create_airtable() -> Airtable {
        let token = std::env::var("AIRTABLE_TOKEN").expect("NOTION_TOKEN must be set");
        Airtable::new(&token)
    }
}
