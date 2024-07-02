# Rusty Power Meter
A little Rust binary which reads out power metrics over SML over USB and stores it into a SQLite database.<br>
The binary also hosts a REST-API on Port 3000 which allows reading out the live metrics and querying stored metrics using SQL statements.

### Project Status
I developed this project over just two weekends in March 2024 and it is not finished nor maintained.
It still runs on my Raspberry Pi after 4+ months and no restarts though, so I think it is useful enough to be shared.

## Quick Start
1. Place the binary on a device which is connected to a USB IR reader which reads the power meter.
2. Check the device path of the USB IR reader (e.g. /dev/ttyUSB0).
```bash
./rusty-power-meter list-ports
```
3. Start the binary with the device path.
```bash
./rusty-power-meter start --path /dev/ttyUSB0
```
4. Enjoy

### Server
The REST-API is hosted on Port 3000. The following endpoints are available:
- GET / - Shows status of the server
- GET /now - Current metrics
- GET /api/now - JSON formatted metrics
- POST /api/query - Query metrics using an SQL statement in the body. (readonly)

### Database
Available columns:
- MeterTime
- Timestamp
- MeterReading
- LineOne
- LineTwo
- LineThree