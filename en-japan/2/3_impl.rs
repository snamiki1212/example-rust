struct Celsius(f64);
struct Kelvin(f64);

impl Celsius {
  fn to_kelvin(self) -> Kelvin {
    return Kelvin(self.0 + 273.15);
  }

  fn from_kelvin(k: Kelvin) -> Self {
    return Celsius(k.0 - 273.15);
  }
}

fn main (){
  let absolute_zero = Kelvin(0.0);
  let triple_point = Celsius(0.0);

  let celsius = Celsius::from_kelvin(absolute_zero);
  let kelvin = triple_point.to_kelvin();

  println!("celsius: {}", celsius.0);
  println!("kelvin: {}", kelvin.0);
}