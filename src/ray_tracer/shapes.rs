pub struct Ray {
    pub origin: (f32, f32, f32),
    pub direction: (f32, f32, f32)
}

pub trait Volume {
    fn get_intersection(&self, ray: &Ray) -> Option<f32>;
    fn color(&self) -> (u8, u8, u8);
}

pub struct Sphere {
    pub color: (u8, u8, u8),
    pub center: (f32, f32, f32),
    pub radius: f32,
}

impl Volume for Sphere {
    fn get_intersection(&self, ray: &Ray) -> Option<f32> {
        let oc = (
            self.center.0 - ray.origin.0,
            self.center.1 - ray.origin.1,
            self.center.2 - ray.origin.2
        );
        let b = 2.0 * (ray.direction.0 * oc.0 + ray.direction.1 * oc.1 + ray.direction.2 * oc.2);
        let c = oc.0 * oc.0 + oc.1 * oc.1 + oc.2 * oc.2 - self.radius * self.radius;
        let disc = b * b - 4.0 * c;
        if disc < 0.0 {
            None
        } else {
            let t = (-b - disc.sqrt()) / 2.0;
            Some(t)
        }
    }
    fn color(&self) -> (u8, u8, u8) {
        self.color
    }
}