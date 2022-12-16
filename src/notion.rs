use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    results: Vec<Page>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Page {
    id: String,
    properties: HashMap<String, Property>,
}

impl Page {
    fn get(&self, key: &str) -> &Property {
        self.properties
            .get(key)
            .map(|p| p.clone())
            .unwrap_or(&Property::NotPresent {})
    }

    pub fn number(&self, key: &str) -> Option<f64> {
        self.get(key).number()
    }

    pub fn string(&self, key: &str) -> Option<String> {
        self.get(key).string()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
enum Property {
    #[serde(rename = "title")]
    Title {
        title: Vec<TitleValue>,
    },
    #[serde(rename = "number")]
    Number {
        number: Option<f64>,
    },
    #[serde(rename = "date")]
    Date {
        date: Date,
    },
    #[serde(rename = "multi_select")]
    MultiSelect {},
    #[serde(rename = "select")]
    Select {
        select: SelectValue,
    },
    #[serde(rename = "relation")]
    Relation {},
    #[serde(rename = "formula")]
    Formula {
        formula: FormulaResult,
    },
    #[serde(rename = "url")]
    Url {
        url: String,
    },
    #[serde(rename = "created_time")]
    CreatedTime {},
    #[serde(rename = "rich_text")]
    RichText {
        rich_text: Vec<RichTextValue>,
    },
    NotPresent,
}

impl Property {
    fn number(&self) -> Option<f64> {
        match self {
            Property::Number { number } => *number,
            Property::Formula { formula } => match formula {
                FormulaResult::Number { number } => Some(*number),
            },
            _ => None,
        }
    }

    fn string(&self) -> Option<String> {
        match self {
            Property::Title { title } => title.get(0).map(|t| t.plain_text.clone()),
            Property::Url { url } => Some(url.clone()),
            Property::Date { date } => date.start.clone(),
            Property::Select { select } => Some(select.name.clone()),
            Property::RichText { rich_text } => rich_text.get(0).map(|t| t.plain_text.clone()),
            _ => None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
enum FormulaResult {
    #[serde(rename = "number")]
    Number { number: f64 },
}

#[derive(Serialize, Deserialize, Debug)]
struct TitleValue {
    plain_text: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct RichTextValue {
    plain_text: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Date {
    start: Option<String>,
    end: Option<String>,
    time_zone: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct SelectValue {
    name: String,
    color: String,
}

pub struct Notion {
    token: String,
}

impl Notion {
    pub fn new(token: &str) -> Self {
        Self {
            token: token.to_string(),
        }
    }

    pub async fn get_database(self, database_id: &str) -> Vec<Page> {
        let client = reqwest::Client::new();
        let res = client
            .post(format!(
                "https://api.notion.com/v1/databases/{}/query",
                database_id
            ))
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Notion-Version", "2021-08-16")
            .send()
            .await
            .unwrap();

        res.json::<Response>().await.unwrap().results
        // println!("{}", res.text().await.unwrap());
        // return vec![];
    }
}
