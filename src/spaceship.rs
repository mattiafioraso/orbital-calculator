use super::orbit::Orbit;

pub struct Spaceship<'name, 'orb> {
  pub name: &'name str,
  pub orbit: Option<&'orb Orbit<'orb>>,
}

impl Spaceship<'_, '_> {
  pub fn calculate_velocity(&self, distance: f64) -> f64 {
    return match self.orbit {
      Some(orb) => (orb.object.spg
        * ((2.0 / (orb.object.radius + distance)) - (1.0 / orb.semimajor_axis)))
        .sqrt(),
      None => 0.0,
    };
  }
}
