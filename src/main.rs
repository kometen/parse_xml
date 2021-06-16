#![allow(non_snake_case)]

mod structs;

use std::fs;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_xml_rs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Goodbye XML!");

    let filename = "./sample.xml";
    let content= fs::read_to_string(filename).expect("Unable to read file");

    let d2LogicalModel: structs::D2LogicalModel =  serde_xml_rs::from_str(&*content).unwrap();

    let mut measurements: Vec<structs::WeatherMeasurement> = Vec::new();

    for site in &d2LogicalModel.payloadPublication.siteMeasurements {
        // The actual weather data
        let mut readings: Vec<structs::Data> = Vec::new();

        let measurement_time_default = &site.measurementTimeDefault.measurementTimeDefault;
        let id = &site.measurementSiteReference.id;

        for measured_value in &site.measuredValue {
            let index = &measured_value.index;
            let weather_node = &measured_value.measuredValue.basicData;

            // precipitationIntensity
            let field_description = &weather_node
                .precipitationDetail
                .precipitationIntensity
                .field_description;
            if !field_description.is_empty() {
                let measurement = &weather_node
                    .precipitationDetail
                    .precipitationIntensity
                    .millimetresPerHourIntensity
                    .millimetresPerHourIntensity;
                let r = structs::Data {
                    index: *index,
                    field_description: field_description.clone(),
                    measurement: *measurement,
                };
                readings.push(r);
                /*println!("measurement time-default: {}, id: {}, index: {}, field description: {}, measurement: {}",
                measurement_time_default, id, index, field_description, measurement);*/
            };

            // windSpeed
            let field_description = &weather_node.wind.windSpeed.field_description;
            if !field_description.is_empty() {
                let measurement = &weather_node.wind.windSpeed.speed.speed;
                let r = structs::Data {
                    index: *index,
                    field_description: field_description.clone(),
                    measurement: *measurement,
                };
                readings.push(r);
                /*println!("measurement time-default: {}, id: {}, index: {}, field description: {}, measurement: {}",
                measurement_time_default, id, index, field_description, measurement);*/
            };

            // airTemperature
            let field_description = &weather_node.temperature.airTemperature.field_description;
            if !field_description.is_empty() {
                let measurement = &weather_node
                    .temperature
                    .airTemperature
                    .temperature
                    .temperature;
                let r = structs::Data {
                    index: *index,
                    field_description: field_description.clone(),
                    measurement: *measurement,
                };
                readings.push(r);
                /*println!("measurement time-default: {}, id: {}, index: {}, field description: {}, measurement: {}",
                measurement_time_default, id, index, field_description, measurement);*/
            };

        }

        let wm = structs::WeatherMeasurement {
            measurement_time_default: measurement_time_default.clone(),
            id: *id,
            data: readings,
        };
        measurements.push(wm);
        // Add final struct here
    }

    let jm = serde_json::to_string(&measurements)?;
    println!("{:?}", &jm);

    Ok(())
}
