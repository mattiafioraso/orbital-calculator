use super::astronomical_object::AstronomicalObject;

pub struct Orbit<'obj> {
  pub object: &'obj AstronomicalObject<'obj>,
  pub apoapsis: f64,
  pub periapsis: f64,
  pub semimajor_axis: f64,
  pub semiminor_axis: f64,
}

pub fn create_orbit<'obj>(
  object: &'obj AstronomicalObject<'obj>,
  apoapsis: f64,
  periapsis: f64,
) -> Orbit<'obj> {
  return Orbit {
    object,
    apoapsis,
    periapsis,
    semimajor_axis: ((object.radius + apoapsis) + (object.radius + periapsis)) / 2.0,
    semiminor_axis: ((object.radius + apoapsis) * (object.radius + periapsis)).sqrt(),
  };
}
