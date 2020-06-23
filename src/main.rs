mod astronomical_object;
mod constants;
mod orbit;
mod spaceship;

use astronomical_object::create_astronomical_object;
use orbit::create_orbit;
use spaceship::Spaceship;

fn main() {
  let earth = create_astronomical_object(
    5.9722 * 10_f64.powi(24),
    6371.0,
    Some("Earth"),
    Some(398600.4418),
  );

  let orbit = create_orbit(&earth, 410.0, 408.0);

  let spaceship = Spaceship {
    name: "Enterprise",
    orbit: Some(&orbit),
  };

  println!(
    "velocity at periapsis: {}",
    spaceship.calculate_velocity(orbit.periapsis)
  );

  println!(
    "velocity at apoapsis: {}",
    spaceship.calculate_velocity(orbit.apoapsis)
  );
}
