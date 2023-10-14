use reqwest::blocking::Request;

fn main() {
    println!("Hello, world!");
    let the_file = r#"{
        "FirstName": "John",
        "LastName": "Doe",
        "Age": 43,
        "Address": {
            "Street": "Downing Street 10",
            "City": "London",
            "Country": "Great Britain"
        },
        "PhoneNumbers": [
            "+44 1234567",
            "+44 2345678"
        ]
    }"#;

    let json: serde_json::Value =
        serde_json::from_str(the_file).expect("bad JSON formatting!");

    println!("{:?}", json);

    let response = reqwest::blocking::get("https://httpbin.org/ip").expect("request failed");

    println!("{:?}", response);

}
