use crate::*;

pub struct Dielectric {
    ref_idx: f32,
}

impl Dielectric {
    pub fn new(ref_idx: f32) -> Dielectric {
        Dielectric { ref_idx }
    }
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r = r * r;
    r + (1.0 - r) * (1.0 - cosine).powf(5.0)
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vector3, Ray)> {
        let dt = hit_record.normal.dot(ray_in.direction());
        let (out_normal, ni_over_nt) = if dt > 0.0 {
            (-&hit_record.normal, self.ref_idx)
        } else {
            (hit_record.normal, 1_f32 / self.ref_idx)
        };
        Some((
            Vector3::new(1.0, 1.0, 1.0),
            Ray::new(
                &hit_record.position,
                &match ray_in.direction().refract(&out_normal, ni_over_nt) {
                    Some(refracted)
                        if Random::gen::<f32>()
                            >= schlick(
                                if dt > 0.0 { self.ref_idx * dt } else { -dt },
                                self.ref_idx,
                            ) =>
                    {
                        refracted
                    }
                    _ => ray_in.direction().reflect(&out_normal),
                },
                ray_in.time(),
            ),
        ))
    }
}
