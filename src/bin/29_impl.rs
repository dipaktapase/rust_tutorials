// The impl keyword is primarily used to define implementations on types. 
// Inherent implementations are standalone, while trait implementations are used to implement traits for types, or other traits.


struct Temperature {
    degrees_f: f64, 
}

impl Temperature {
    fn freezing() -> Self { // Self refers to the Teperature on line 5th.
        Self {degrees_f: 32.0}
    }   

    fn boiling() -> Self {
        Self {degrees_f: 150.0}
    }

    fn show_temp(&self) { // Ampersand (&) means borrowing, In this case self is just Temperature. This self refers to the Temperature on line 1st i.e. struct.
        println!("{:?} degrees F", self.degrees_f); // here self is just going to refer to the temperature
    }
}

// Difference between Self and self

fn main() {
    let hot = Temperature { degrees_f: 99.9 };
    // Temperature::show_temp(hot)
    hot.show_temp();
    
    let cold = Temperature::freezing();
    cold.show_temp();

    let boil = Temperature::boiling();
    boil.show_temp();
}