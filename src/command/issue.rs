use chrono::NaiveDate;
use std::fs::File;
use std::io::prelude::*;

pub async fn generate_issue(number: i64) {
    let airtable = crate::airtable::Airtable::new(
        &std::env::var("AIRTABLE_TOKEN").expect("AIRTABLE_TOKEN must be set"),
    );
    let issues = airtable.records("appxauMzM76PEp2Aw", "Issues").await;

    let issue = issues
        .iter()
        .find(|issue| issue.integer("Issue") == Some(number))
        .unwrap();

    let article_ids = issue.records("Articles");

    let mut links: Vec<Link> = vec![];

    for id in article_ids {
        let airtable = crate::airtable::Airtable::new(
            &std::env::var("AIRTABLE_TOKEN").expect("AIRTABLE_TOKEN must be set"),
        );
        let record = airtable
            .record("appxauMzM76PEp2Aw", "Articles", &id)
            .await
            .unwrap();

        let link_type = match record.string("Type").unwrap().as_str() {
            "Video" => Type::Video,
            "Article" => Type::Article,
            "News" => Type::News,
            _ => panic!("Unknown type"),
        };

        let link = Link::new(
            record.string("Name").unwrap(),
            record.string("Url").unwrap(),
            record.string("Description").unwrap_or("".to_string()),
            link_type,
        );
        links.push(link);
    }

    let published =
        NaiveDate::parse_from_str(&issue.string("Published").unwrap(), "%Y-%m-%d").unwrap();
    let date = published.format("%Y-%m-%d").to_string();
    let number = issue.integer("Issue").unwrap();
    let title = format!("Issue #{} - {}", number, published.format("%B %d, %Y"));

    let news = links_to_string(
        links
            .iter()
            .filter(|link| link.is == Type::News)
            .cloned()
            .collect(),
    );
    let articles = links_to_string(
        links
            .iter()
            .filter(|link| link.is == Type::Article)
            .cloned()
            .collect(),
    );
    let videos = links_to_string(
        links
            .iter()
            .filter(|link| link.is == Type::Video)
            .cloned()
            .collect(),
    );

    let body = format!(
        r#"+++
title="{title}"
date="{date}"
[extra]
issue={number}
+++
{articles}

### 📰 News
{news}

### 📺 Videos
{videos}
"#
    );

    let mut file = File::create(format!("./website/content/{}.md", number)).unwrap();
    file.write_all(body.as_bytes()).unwrap();
    println!("{:?}", title);
}

#[derive(Debug, PartialEq, Clone)]
enum Type {
    Video,
    Article,
    News,
}

#[derive(Debug, Clone)]
struct Link {
    name: String,
    url: String,
    description: String,
    is: Type,
}

impl Link {
    fn new(name: String, url: String, description: String, is: Type) -> Self {
        Self {
            name,
            url,
            description,
            is,
        }
    }
}

fn links_to_string(links: Vec<Link>) -> String {
    links.iter().fold(String::new(), |mut acc, link| {
        acc.push_str(&format!(
            "**[{}]({})** {}\n\n",
            link.name, link.url, link.description
        ));
        acc
    })
}
