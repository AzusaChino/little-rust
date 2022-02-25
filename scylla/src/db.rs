use anyhow::Result;
use scylla::{IntoTypedRows, Session, SessionBuilder};
use uuid::Uuid;

use crate::Duration;
use crate::TemperatureMeasurement;

pub async fn create_session(uri: &str) -> Result<Session> {
    SessionBuilder::new()
        .known_node(uri)
        .build()
        .await
        .map_err(From::from)
}

static CREATE_KEYSPACE_QUERY: &str = r#"
  CREATE KEYSPACE IF NOT EXISTS tutorial
    WITH REPLICATION = {
      'class': 'SimpleStrategy',
      'replication_factor': 1
    };
"#;

static CREATE_TEMPERATURE_TABLE_QUERY: &str = r#"
  CREATE TABLE IF NOT EXISTS tutorial.temperature (
    device UUID,
    time timestamp,
    temperature smallint,
    PRIMARY KEY(device, time)
  );
"#;

pub async fn initialize(session: &Session) -> Result<()> {
    create_keyspace(session).await?;
    create_temperature_table(session).await?;
    Ok(())
}


async fn create_keyspace(session: &Session) -> Result<()> {
    session
        .query(CREATE_KEYSPACE_QUERY, ())
        .await
        .map(|_| ())
        .map_err(From::from)
}

async fn create_temperature_table(session: &Session) -> Result<()> {
    session
        .query(CREATE_TEMPERATURE_TABLE_QUERY, ())
        .await
        .map(|_| ())
        .map_err(From::from)
}

static ADD_MEASUREMENT_QUERY: &str = r#"
  INSERT INTO tutorial.temperature (device, time, temperature)
    VALUES (?, ?, ?);
"#;

pub async fn add_measurement(session: &Session, measurement: TemperatureMeasurement) -> Result<()> {
    session
        .query(ADD_MEASUREMENT_QUERY, measurement)
        .await
        .map(|_| ())
        .map_err(From::from)
}

static SELECT_MEASUREMENTS_QUERY: &str = r#"
  SELECT * FROM fast_logger.temperature
    WHERE device = ?
      AND time > ?
      AND time < ?;
"#;

pub async fn select_measurements(
    session: &Session,
    device: Uuid,
    time_from: Duration,
    time_to: Duration,
) -> Result<Vec<TemperatureMeasurement>> {
    session
        .query(SELECT_MEASUREMENTS_QUERY, (device, time_from, time_to))
        .await?
        .rows
        .unwrap_or_default()
        .into_typed::<TemperatureMeasurement>()
        .map(|v| v.map_err(From::from))
        .collect()
}
