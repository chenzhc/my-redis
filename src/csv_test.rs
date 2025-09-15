#![cfg_attr( debug_assertions,
    allow(dead_code, unused_imports, unused_variables, unused_mut)
)]
#![allow(dead_code,unused_variables)]

#[derive(Debug,serde::Deserialize, serde::Serialize)]
pub struct Vehicle {
    #[serde(rename(deserialize = "Manufacturer"))]
    manufacturer: String,
    #[serde(rename(deserialize = "Model"))]
    model: String, 
    #[serde(rename(deserialize = "VIN"))]
    vin: String,
}


#[cfg(test)]
mod tests {
    use csv::ReaderBuilder;
    use log::info;
    use log4rs::append::file;
    use super::*;

    #[tokio::test(flavor="multi_thread")]
    async fn it_piple_test() {
        crate::init();

        let file_name = "data.csv";
        let mut builder = ReaderBuilder::new();
        builder.double_quote(false)
            .comment(Some(b'-'))
            .delimiter(b'|')
            .has_headers(true);
        let my_reader = builder .from_path(file_name);

        // let my_reader = csv::Reader::from_path(file_name);

        if my_reader.is_err() {
            info!("Failed to read CSV. File path probably doesn't exist, or you don't have permissions.");
            return;
        }

        let mut reader = my_reader.unwrap();
        for record in reader.deserialize() {
            let car: Vehicle = record.unwrap();
            info!("Your car manufacturer is {}", car.manufacturer); 
            info!("Your car model is {}", car.model);
            info!("Your car VIN is {}", car.vin);
        }

    }

}