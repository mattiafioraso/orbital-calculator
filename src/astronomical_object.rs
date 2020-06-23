use super::constants::G;

pub struct AstronomicalObject<'name> {
  /// The astronomical object name
  pub name: Option<&'name str>,
  /// The astronomical object radius in km
  pub radius: f64,
  /// The astronomical object mass in kg
  pub mass: f64,
  /// The astronomical object standard gravitational parameter in km³⋅s⁻²
  pub spg: f64,
}

pub fn create_astronomical_object(
  mass: f64,
  radius: f64,
  name: Option<&str>,
  spg: Option<f64>,
) -> AstronomicalObject {
  return AstronomicalObject {
    radius,
    mass,
    spg: match spg {
      Some(s) => s,
      None => (G * mass) / 10_f64.powi(9),
    },
    name,
  };
}
