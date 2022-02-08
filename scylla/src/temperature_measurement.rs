use scylla::{FromRow, ValueList};
use uuid::Uuid;

use crate::Duration;

#[derive(Debug, FromRow, ValueList)]
pub struct TemperatureMeasurement {
    pub device: Uuid,
    pub time: Duration,
    pub temperature: i16,
}
