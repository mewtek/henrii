use std::collections::HashMap;

pub fn get_dtcs() -> HashMap<&'static str, &'static str> {
    let diagnostic_trouble_codes: HashMap<&str, &str> = [
        ("P0001", "Fuel Volume Regulator Control Circuit/Open"),
        ("P0002", "Fuel Volume Regulator Control Circuit Range/Performance Problem"),
        ("P0010", "A Camshaft Position Actuator Circuit (Bank 1)"),
        ("P0011", "A Camshaft Position Timing Over-Advanced or System Performance (Bank 1)"),
        ("P0012", "A Camshaft Position Timing Over-Retarded (Bank 1)"),
        ("P0013", "B Camshaft Position Actuator Circuit (Bank 1)"),
        ("P0014", "B Camshaft Position Timing Over-Advanced or System Performance (Bank 1)"),
        ("P0015", "B Camshaft Position Timing Over-Retarded (Bank 1)"),
        ("P0020", "A Camshaft Position Actuator Circuit (Bank 2)"),
        ("P0021", "A Camshaft Position Timing Over-Advanced or System Performance (Bank 2)"),
        ("P0022", "A Camshaft Position Timing Over-Retarded (Bank 2)"),
        ("P0300", "Random/Multiple Cylinder Misfire Detected"),
        ("P0301", "Cylinder 1 Misfire Detected"),
        ("P0302", "Cylinder 2 Misfire Detected"),
        ("P0303", "Cylinder 3 Misfire Detected"),
        ("P0304", "Cylinder 4 Misfire Detected"),
        ("P0305", "Cylinder 5 Misfire Detected"),
        ("P0306", "Cylinder 6 Misfire Detected"),
        ("P0420", "Catalyst System Efficiency Below Threshold (Bank 1)"),
        ("P0430", "Catalyst System Efficiency Below Threshold (Bank 2)"),
        ("P0440", "Evaporative Emission Control System Malfunction"),
        ("P0441", "Evaporative Emission Control System Incorrect Purge Flow"),
        ("P0442", "Evaporative Emission Control System Leak Detected (Small Leak)"),
        ("P0443", "Evaporative Emission Control System Purge Control Valve Circuit"),
        ("P0450", "Evaporative Emission Control System Pressure Sensor Malfunction"),
        ("P0455", "Evaporative Emission Control System Leak Detected (Large Leak)"),
        ("P0500", "Vehicle Speed Sensor A"),
        ("P0501", "Vehicle Speed Sensor Range/Performance Problem"),
        ("P0700", "Transmission Control System (MIL Request)"),
        ("P0710", "Transmission Fluid Temperature Sensor Circuit"),
        ("P0711", "Transmission Fluid Temperature Sensor Range/Performance Problem"),
        ("P0800", "Transmission Control System Malfunction"),
        ("P1130", "Fuel Trim Malfunction (Bank 1)"),
        ("P1340", "VTEC System Malfunction"),
        ("P1600", "ECM/PCM Internal Malfunction"),
        ("P2195", "O2 Sensor Signal Stuck Lean (Bank 1)"),
        ("P2196", "O2 Sensor Signal Stuck Rich (Bank 1)"),
        ("P2270", "O2 Sensor Signal Stuck Lean (Bank 2)"),
        ("P2271", "O2 Sensor Signal Stuck Rich (Bank 2)"),
        ("P2610", "Engine Coolant Temperature Sensor Circuit Range/Performance Problem"),
        ("P2611", "Engine Coolant Temperature Sensor Circuit Range/Performance Problem"),
        ("P3000", "Hybrid Battery Pack Voltage System Malfunction"),
        ("P3001", "Hybrid Battery Pack Voltage System Range/Performance Problem"),
        ("P3002", "Hybrid Battery Pack Voltage System Low"),
        ("P3003", "Hybrid Battery Pack Voltage System High"),
        ("P3004", "Hybrid Battery Pack Voltage System Discharge"),
        ("P3005", "Hybrid Battery Pack Voltage System Charge"),
    ]
    .iter()
    .cloned()
    .collect();

    return diagnostic_trouble_codes;
}