use dotenv::dotenv;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use reqwest::header::USER_AGENT;
use scraper::{Html, Selector};
use serde_json::Value;
use std::error::Error;
use std::io;

pub async fn get_tui_price(url: &str) -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::builder().build()?;
    let request = client
        .request(reqwest::Method::GET, url)
        .header(USER_AGENT, "curl");
    let response = request.send().await?.text().await?;
    // Parse the HTML content
    let document = Html::parse_document(&response);
    // Parse all the scripts in the HTML content
    let selector = Selector::parse("script").map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to parse selector: {}", e),
        )
    })?;
    let scripts = document.select(&selector);
    for script in scripts {
        let script_text = script.text().collect::<String>();
        // Look for "currentPrice" in the script
        if script_text.contains("currentPrice") {
            // Extract the JSON object from the script
            if let (Some(json_start), Some(json_end)) =
                (script_text.find('{'), script_text.rfind('}'))
            {
                let json = &script_text[json_start..json_end + 1];
                // Parse the value with key "currentPrice"
                match serde_json::from_str::<Value>(json) {
                    Ok(value) => {
                        if let Some(product) = value.get("product") {
                            if let Some(price) = product.get("currentPrice") {
                                return Ok(price.to_string());
                            } else {
                                return Err(Box::new(io::Error::new(
                                    io::ErrorKind::Other,
                                    "currentPrice key not found",
                                )));
                            }
                        } else {
                            return Err(Box::new(io::Error::new(
                                io::ErrorKind::Other,
                                "product key not found",
                            )));
                        }
                    }
                    Err(e) => {
                        return Err(Box::new(io::Error::new(
                            io::ErrorKind::Other,
                            format!("Failed to parse JSON: {}", e),
                        )));
                    }
                }
            }
        }
    }

    Err(Box::new(io::Error::new(
        io::ErrorKind::Other,
        "Script containing currentPrice not found",
    )))
}

pub fn send_email(price: &str, old_price: &str, email_addr: &str) -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let cred_password = std::env::var("CRED_PASSWORD")?;
    let cred_email = std::env::var("CRED_EMAIL")?;
    let email = std::env::var("EMAIL")?;
    let msg = Message::builder()
        .from(format!("TUIWatcher <{}>", email).parse()?)
        .to(email_addr.parse::<lettre::message::Mailbox>()?)
        .subject("The price has changed")
        .body(format!(
            "The price has changed from {} to {}!",
            old_price, price,
        ))
        .unwrap();
    let creds = Credentials::new(cred_email.to_string(), cred_password.to_string());
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&msg) {
        Err(e) => Err(Box::new(e)),
        _ => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_tui_price() {
        let url =
            "https://www.tui.fi/fi/varaa-matka?productCode=P-000008936&tab=overview&noOfAdults=2&noOfChildren=0&childrenAge=&duration=14&flexibleDays=3&airports%5B%5D=HEL&flexibility=true&noOfSeniors=0&when=31-07-2024&pkg=5390643510/3/577/14&tra_o=A2d93e6345a84d0906885aa097530b74f&tra_i=A2d93e6345a84d0906885aa097530b74f&units%5B%5D=G-000000293:DESTINATION&packageId=P-000008936GRRH006517223840000001722384000000AY186317235936000001723680000000AY1864DD015390643510/3/577/14&index=2&multiSelect=true&brandType=L&bb=AI&room=&isVilla=false&searchType=search&durationCode=1413&rmpc=1|2|0|0|0|&rmtp=DD01&rmbb=AI&fc=n&greatDealDiscount=0&bb=AI&price=pp";
        let res = get_tui_price(url).await.unwrap();
        assert_eq!(res, "5642.0");
    }

    #[test]
    fn test_send_email() {
        let email = "nooa.kronqvist@hotmail.com".to_string();
        assert!(send_email("100", "150", &email).is_ok());
    }
}
