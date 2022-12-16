use lettre::{transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};

pub async fn send_newsletter(_number: i64) {
    let airtable = crate::airtable::Airtable::new(
        &std::env::var("AIRTABLE_TOKEN").expect("AIRTABLE_TOKEN must be set"),
    );
    let subscribers = airtable.records("appxauMzM76PEp2Aw", "Subscribers").await;

    println!("{:?}", subscribers);
    println!("Sending newsletter to {} subscribers", subscribers.len());

    let smtp_credentials = Credentials::new("".to_string(), "".to_string());

    let mailer = SmtpTransport::relay("email-smtp.us-east-1.amazonaws.com")
        .unwrap()
        .credentials(smtp_credentials)
        .build();

    let email = Message::builder()
        .from(
            "RockClimbing News <info@rockclimbingnews.com>"
                .parse()
                .unwrap(),
        )
        .to("edelprino@gmail.com <edelprino@gmail.com>".parse().unwrap())
        .subject("Issue e")
        .body("Prova di un test".to_string())
        .unwrap();

    mailer.send(&email).unwrap();
}
