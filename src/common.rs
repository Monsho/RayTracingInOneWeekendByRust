pub struct RandValue;

impl RandValue {
    pub fn get() -> f64 {
        rand::random()
    }

    pub fn get_range(v_min : f64, v_max : f64) -> f64 {
        let t = RandValue::get();
        v_min + (v_max - v_min) * t
    }
}

