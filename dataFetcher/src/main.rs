use std::{error::Error, fs::OpenOptions, io::Write, thread, time::Duration};
use chrono::Utc;
use serde_json::Value;

// Define the Pricing trait with methods to fetch a price and save it to a file.
trait Pricing {
    fn fetch_price(&mut self) -> Result<f64, Box<dyn Error>>;
    fn save_to_file(&self) -> Result<(), Box<dyn Error>>;
    fn name(&self) -> &str;
}

// Struct representing Bitcoin pricing data.
struct Bitcoin {
    price: f64,
    last_updated: String,
}

impl Pricing for Bitcoin {
    fn fetch_price(&mut self) -> Result<f64, Box<dyn Error>> {
        // API endpoint from CoinGecko for Bitcoin price in USD
        let response = ureq::get("https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd")
            .call()?
            .into_string()?;
        
        // Parse JSON response; expected format: {"bitcoin":{"usd":<price>}}
        let v: Value = serde_json::from_str(&response)?;
        let price = v["bitcoin"]["usd"]
            .as_f64()
            .ok_or("Price not found in response")?;
        
        self.price = price;
        self.last_updated = Utc::now().to_rfc3339();
        Ok(price)
    }

    fn save_to_file(&self) -> Result<(), Box<dyn Error>> {
        // Append the fetched price to "bitcoin.txt"
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("bitcoin.txt")?;
        writeln!(file, "{}: ${}", self.last_updated, self.price)?;
        Ok(())
    }

    fn name(&self) -> &str {
        "Bitcoin"
    }
}

// Struct representing Ethereum pricing data.
struct Ethereum {
    price: f64,
    last_updated: String,
}

impl Pricing for Ethereum {
    fn fetch_price(&mut self) -> Result<f64, Box<dyn Error>> {
        // API endpoint from CoinGecko for Ethereum price in USD
        let response = ureq::get("https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd")
            .call()?
            .into_string()?;
        
        // Parse JSON response; expected format: {"ethereum":{"usd":<price>}}
        let v: Value = serde_json::from_str(&response)?;
        let price = v["ethereum"]["usd"]
            .as_f64()
            .ok_or("Price not found in response")?;
        
        self.price = price;
        self.last_updated = Utc::now().to_rfc3339();
        Ok(price)
    }

    fn save_to_file(&self) -> Result<(), Box<dyn Error>> {
        // Append the fetched price to "ethereum.txt"
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("ethereum.txt")?;
        writeln!(file, "{}: ${}", self.last_updated, self.price)?;
        Ok(())
    }

    fn name(&self) -> &str {
        "Ethereum"
    }
}

// Struct representing S&P 500 pricing data.
struct SP500 {
    price: f64,
    last_updated: String,
}

impl Pricing for SP500 {
    fn fetch_price(&mut self) -> Result<f64, Box<dyn Error>> {
        // Updated URL with your API key
        let url = "https://query1.finance.yahoo.com/v8/finance/chart/%5EGSPC?interval=1m";
        let response = ureq::get(url)
            .call()?
            .into_string()?;
        
        // Parse the JSON response
        let v: Value = serde_json::from_str(&response)?;
        let price = v[0]["price"]
            .as_f64()
            .ok_or("Price not found in response")?;
        
        self.price = price;
        self.last_updated = Utc::now().to_rfc3339();
        Ok(price)
    }

    fn save_to_file(&self) -> Result<(), Box<dyn Error>> {
        // Append the fetched price to "sp500.txt"
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("sp500.txt")?;
        writeln!(file, "{}: ${}", self.last_updated, self.price)?;
        Ok(())
    }

    fn name(&self) -> &str {
        "SP500"
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Create a vector to store each asset using a trait object for dynamic dispatch.
    let mut assets: Vec<Box<dyn Pricing>> = vec![
        Box::new(Bitcoin {
            price: 0.0,
            last_updated: String::new(),
        }),
        Box::new(Ethereum {
            price: 0.0,
            last_updated: String::new(),
        }),
        Box::new(SP500 {
            price: 0.0,
            last_updated: String::new(),
        }),
    ];

    // The application runs indefinitely, fetching and saving data every 10 seconds.
    loop {
        for asset in assets.iter_mut() {
            match asset.fetch_price() {
                Ok(price) => {
                    println!("Fetched {} price: ${}", asset.name(), price);
                    asset.save_to_file()?;
                }
                Err(e) => {
                    eprintln!("Error fetching {}: {}", asset.name(), e);
                }
            }
        }
        // Pause for 10 seconds before the next fetch cycle.
        thread::sleep(Duration::from_secs(10));
    }
}
