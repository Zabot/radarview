use bevy::math::Vec3;

#[derive(Debug, Clone)]
pub struct PolarVec3 {
    pub range: f32,
    pub azimuth: f32,
    pub elevation: f32,
}

impl PolarVec3 {
    pub fn new(range: f32, azimuth: f32, elevation: f32) -> Self {
        Self {
            range,
            azimuth,
            elevation,
        }
    }

    /// Convert this polar vector to a cartesian vector directly,
    /// without changing the values
    pub fn direct_vec3(&self) -> Vec3 {
        Vec3::new(self.azimuth, self.elevation, self.range)
    }
}

impl From<Vec3> for PolarVec3 {
    fn from(val: Vec3) -> Self {
        let range = val.length();
        Self {
            range,
            azimuth: val.x.atan2(val.z),
            elevation: (val.y / range).asin(),
        }
    }
}

impl From<PolarVec3> for Vec3 {
    fn from(val: PolarVec3) -> Self {
        Self {
            z: (val.range * val.elevation.cos()) * val.azimuth.cos(),
            x: (val.range * val.elevation.cos()) * val.azimuth.sin(),
            y: val.range * val.elevation.sin(),
        }
    }
}

#[cfg(test)]
mod test {
    use std::f32::consts::{FRAC_PI_2, FRAC_PI_4};

    use bevy::math::Vec3;

    use super::PolarVec3;

    #[test]
    fn round_trip() {
        for vec in [
            Vec3::X,
            Vec3::Y,
            Vec3::Z,
            Vec3::NEG_X,
            Vec3::NEG_Y,
            Vec3::NEG_Z,
        ] {
            let polar: PolarVec3 = vec.into();
            let t: Vec3 = polar.into();
            assert!((vec - t).length() < 0.001);
        }

        for vec in [
            PolarVec3::new(200_000.0, FRAC_PI_4, 0.0),
            PolarVec3::new(200_000.0, 0.0, FRAC_PI_4),
            PolarVec3::new(200_000.0, FRAC_PI_4, FRAC_PI_4),
        ] {
            dbg!(&vec);
            let cart: Vec3 = vec.clone().into();
            dbg!(&cart);
            let p: PolarVec3 = cart.into();
            dbg!(&p);
            assert!((vec.direct_vec3() - p.direct_vec3()).length() < 0.001);
        }
    }
}
