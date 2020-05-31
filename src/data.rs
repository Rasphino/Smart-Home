/// Data from multiple devices
pub enum Data {
    /// DHT-11: temperature and humidity
    DHT11(i32, i32),
    /// HC-SR04: distance
    HCSR04(f32),
}