//const variable starting at 32 Farenheit
const freezing: f64 = 32.0;

//F to C
fn fahrenheit_to_celsius(f: f64) -> f64{
    (f - freezing) * 5.0 / 9.0
}

//C to F
fn celsius_to_fahrenheit(c: f64) -> f64{
    c * 9.0 / 5.0 + freezing
}

fn main(){
    //start at 32F
    let mut tempF: f64 = freezing;
    
    //convert the starting temperature
    let tempC = fahrenheit_to_celsius(tempF);
    println!("{:.2}°F is equal to {:.2}°C", tempF, tempC);

    //loop for the next 5 temperatures.
    for i in 1..=5{
        let currentT = tempF + i as f64;
        let currentC = fahrenheit_to_celsius(currentT);
        println!("{:.2}°F is equal to {:.2}°C", currentT, currentC);
    }
}