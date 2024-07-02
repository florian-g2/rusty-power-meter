use std::fmt::Display;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use anyhow::{bail, Error};
use serde::Serialize;
use sqlite::{Connection, ConnectionThreadSafe, OpenFlags, Row, Type};

use crate::meter_reading::MeterReading;
use crate::unit::Unit;

pub struct Database(Connection);

impl Database {
    fn path() -> Result<PathBuf, anyhow::Error> {
        let local_dir = dirs::data_local_dir().ok_or_else(|| anyhow::anyhow!("Could not find user directory."))?;
        let path = local_dir.join("rusty-power-meter").join("database.sqlite3");

        Ok(path)
    }

    fn init(path: &Path) -> Result<Self, anyhow::Error> {
        if path.exists() {
            bail!("Database already exists.")
        }
        
        let connection = Connection::open(path)?;

        let statement = " \
            CREATE TABLE Readings ( \
                MeterTime INTEGER, \
                Timestamp DATETIME NOT NULL, \
                MeterReading REAL NOT NULL, \
                LineOne INTEGER, \
                LineTwo INTEGER, \
                LineThree INTEGER \
            ); \
            CREATE UNIQUE INDEX idx_timestamp ON Readings (Timestamp);
        ";

        connection.execute(statement)?;

        Ok(Self(connection))
    }

    pub fn load() -> Result<Self, anyhow::Error> {
        let path = Self::path()?;

        if path.exists() {
            let connection = Connection::open(&path)?;
            Ok(Self(connection))
        } else {
            fs::create_dir_all(path.parent().unwrap())?;
            Ok(Self::init(&path)?)
        }
    }
    
    pub fn insert_reading(&self, reading: &MeterReading) -> Result<(), anyhow::Error> {
        let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_secs() as i64;
        // let timestamp = sqlite::Value::Binary(timestamp.to_le_bytes().to_vec());
        
        let mut statement = self.0.prepare("INSERT INTO Readings (MeterTime, Timestamp, MeterReading, LineOne, LineTwo, LineThree) VALUES (?, ?, ?, ?, ?, ?)")?;
        statement.bind((1, reading.meter_time.map(|x| x as i64)))?;
        statement.bind((2, timestamp))?;
        statement.bind((3, reading.meter_reading))?;
        statement.bind((4, reading.line_one.map(|x| x as i64)))?;
        statement.bind((5, reading.line_two.map(|x| x as i64)))?;
        statement.bind((6, reading.line_three.map(|x| x as i64)))?;

        let result = statement.next();

        const UNIQUE_CONSTRAINT_ERROR: isize = 19;
        if let Err(error) = result {
            if error.code == Some(UNIQUE_CONSTRAINT_ERROR) {
                println!("Warning: Duplicate timestamp.");
                return Ok(());
            }

            return Err(error.into());
        }
        
        Ok(())
    }
    
    pub fn list_readings<'a>(&'a self) -> Result<impl Iterator<Item = Result<MeterReading, Error>> + 'a, anyhow::Error> {
        let statement = self.0.prepare("SELECT MeterTime, Timestamp, MeterReading, LineOne, LineTwo, LineThree FROM Readings")?;
        
        Ok(statement.into_iter().map(move |row| {
            let Ok(row) = row else {
                return Err(anyhow::anyhow!("Error reading row."));
            };
            
            Ok(MeterReading {
                meter_time: Some(row.read::<i64, _>(0) as u32),
                meter_reading: Some(row.read::<f64, _>(0)),
                meter_reading_unit: Some(Unit::WattHour),
                line_one: Some(row.read::<i64, _>(1) as i32),
                line_one_unit: Some(Unit::Watt),
                line_two: Some(row.read::<i64, _>(2) as i32),
                line_two_unit: Some(Unit::Watt),
                line_three: Some(row.read::<i64, _>(3) as i32),
                line_three_unit: Some(Unit::Watt),
            })
        }))
    }
    
    pub fn metrics(&self) -> Result<DatabaseMetrics, anyhow::Error> {
        let count_stmt = self.0.prepare("SELECT COUNT(*) FROM Readings")?;
        let count_row = count_stmt.into_iter().next().ok_or(anyhow::anyhow!("No count row."))??;
        
        let count_readings = count_row.read::<i64, _>(0) as u64;
        let file_size = fs::metadata(Self::path()?)?.len();
        
        Ok(DatabaseMetrics {
            location: Self::path()?,
            count_readings,
            file_size,
        })
    }
}


pub enum Value {
    // U64(u64),
    I64(i64),
    F64(f64),
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        match self {
            Value::I64(value) => serializer.serialize_i64(*value),
            Value::F64(value) => serializer.serialize_f64(*value),
        }
    }
}

#[derive(Serialize)]
pub struct QueryResult {
    columns: Vec<String>,
    took_ms: u64,
    rows_count: u64,
    rows: Vec<Vec<Option<Value>>>,
}

pub struct ReadonlyDatabase(ConnectionThreadSafe);

impl ReadonlyDatabase {
    pub fn load() -> Result<Self, anyhow::Error> {
        let path = Database::path()?;

        if path.exists() {
            let open_flags = OpenFlags::new().with_read_only();
            let connection = Connection::open_thread_safe_with_flags(&path, open_flags)?;

            Ok(Self(connection))
        } else {
            bail!("Database does not exist.")
        }
    }

    pub fn query(&self, statement: &str) -> Result<QueryResult, anyhow::Error> {
        let mut statement = self.0.prepare(statement)?;

        let query_start = SystemTime::now();
        let column_names = statement.column_names().to_vec();
        let column_count = statement.column_count();
        
        // execute statement.
        statement.next()?;

        // after the statement has been executed, the column types are available.
        let mut column_types = Vec::<Type>::with_capacity(column_count);
        for index in 0..column_count {
            column_types.push(statement.column_type(index)?);
        }

        let mut rows = Vec::<Vec<Option<Value>>>::new();
        for row in statement.into_iter() {
            let row = row?;
            let mut values = Vec::<Option<Value>>::with_capacity(column_count);

            for index in 0..column_count {
                let value = match column_types[index] {
                    Type::Integer => Some(Value::I64(row.read::<i64, usize>(index))),
                    Type::Float => Some(Value::F64(row.read::<f64, _>(index))),
                    // Type::Binary => {
                    //     let bytes = row.read::<&[u8], _>(index);
                    //     let value = u64::from_le_bytes(bytes.try_into().unwrap());
                    //     
                    //     Some(Value::U64(value))
                    // }
                    Type::Null => None,
                    _ => {
                        bail!("Unexpected column type \"{:?}\".", column_types[index]);
                    }
                };

                values.push(value);
            }
            
            rows.push(values);
        }

        let duration = SystemTime::now().duration_since(query_start)?.as_millis() as u64;
        let query_result = QueryResult {
            columns: column_names,
            rows_count: rows.len() as u64,
            rows,
            took_ms: duration,
        };

        Ok(query_result)
    }
}


pub struct DatabaseMetrics {
    pub location: PathBuf,
    pub count_readings: u64,
    pub file_size: u64,
}

impl Display for DatabaseMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Location: {}\n", self.location.display())?;
        write!(f, "Metrics: {} readings, {} bytes", self.count_readings, self.file_size)
    }
}