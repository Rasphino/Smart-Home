/// Data from multiple devices
pub enum Data {
    /// DHT-11: temperature and humidity
    DHT11(u32, u32),
    /// HC-SR04: distance
    HCSR04(f32),
}