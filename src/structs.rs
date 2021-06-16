// XML-root
#[derive(Deserialize, Debug)]
pub(crate) struct D2LogicalModel {
    pub(crate) payloadPublication: PayloadPublication,
}

// payloadPublication
#[derive(Deserialize, Debug)]
pub(crate) struct PayloadPublication {
    lang: String,
    pub(crate) publicationTime: PublicationTime,
    pub(crate) siteMeasurements: Vec<SiteMeasurements>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct PublicationTime {
    #[serde(rename = "$value")]
    pub(crate) publicationTime: String,
}

// // siteMeasurements, the various weather measurements are below (sub-root).
#[derive(Deserialize, Debug)]
pub(crate) struct SiteMeasurements {
    pub(crate) measurementSiteReference: MeasurementSiteReference,
    pub(crate) measurementTimeDefault: MeasurementTimeDefault,
    #[serde(default)]
    pub(crate) measuredValue: Vec<MeasuredValue_>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct MeasurementSiteReference {
    pub(crate) id: u16,
    targetClass: String,
    version: u16,
}

#[derive(Deserialize, Debug)]
pub(crate) struct MeasurementTimeDefault {
    #[serde(rename = "$value")]
    pub(crate) measurementTimeDefault: String,
}

// Common for all measurements.
#[derive(Deserialize, Debug)]
pub(crate) struct MeasuredValue_ {
    pub(crate) index: u16,
    pub(crate) measuredValue: MeasuredValue,
}

#[derive(Deserialize, Debug)]
pub(crate) struct MeasuredValue {
    pub(crate) basicData: BasicData,
}

// Split based on type of measurement. Below this point the tree is different.
#[derive(Deserialize, Debug, Default)]
pub(crate) struct BasicData {
    #[serde(default)]
    pub(crate) precipitationDetail: PrecipitationDetail,
    #[serde(default)]
    pub(crate) wind: Wind,
    #[serde(default)]
    pub(crate) temperature: Temperature_, // Add underscore since this collides with another struct
}

// precipitationIntensity
#[derive(Deserialize, Debug, Default)]
pub(crate) struct PrecipitationDetail {
    #[serde(default)]
    pub(crate) precipitationIntensity: PrecipitationIntensity,
}

#[derive(Deserialize, Debug, Default)]
pub(crate) struct PrecipitationIntensity {
    #[serde(default = "precipitation_intensity")]
    pub(crate) field_description: String,
    #[serde(default)]
    pub(crate) millimetresPerHourIntensity: MillimetresPerHourIntensity,
}

#[derive(Deserialize, Debug, Default)]
pub(crate) struct MillimetresPerHourIntensity {
    #[serde(rename = "$value")]
    pub(crate) millimetresPerHourIntensity: f32,
}

#[derive(Deserialize, Debug, Default)]
pub(crate) struct Temperature {
    #[serde(rename = "$value")]
    pub(crate) temperature: f32,
}

// windSpeed
#[derive(Deserialize, Debug, Default)]
pub(crate) struct Wind {
    #[serde(default)]
    pub(crate) windSpeed: WindSpeed,
}

#[derive(Deserialize, Debug, Default)]
pub(crate) struct WindSpeed {
    #[serde(default = "wind_speed")]
    pub(crate) field_description: String,
    #[serde(default)]
    pub(crate) speed: Speed,
}

#[derive(Deserialize, Debug, Default)]
pub(crate) struct Speed {
    #[serde(rename = "$value")]
    pub(crate) speed: f32,
}

// airTemperature
#[derive(Deserialize, Debug, Default)]
pub(crate) struct Temperature_ {
    #[serde(default)]
    pub(crate) airTemperature: AirTemperature,
}

#[derive(Deserialize, Debug, Default)]
pub(crate) struct AirTemperature {
    #[serde(default = "air_temperature")]
    pub(crate) field_description: String,
    #[serde(default)]
    pub(crate) temperature: Temperature,
}

// Add default values in serde.
fn precipitation_intensity() -> String {
    "precipitation_intensity".to_string()
}

fn wind_speed() -> String {
    "wind_speed".to_string()
}

fn air_temperature() -> String {
    "air_temperature".to_string()
}

#[derive(Serialize)]
pub(crate) struct WeatherMeasurement {
    pub(crate) measurement_time_default: String,
    pub(crate) id: u16,
    pub(crate) data: Vec<Data>,
}

#[derive(Serialize)]
pub(crate) struct Data {
    pub(crate) index: u16,
    pub(crate) field_description: String,
    pub(crate) measurement: f32,
}
