
fn main() {
    let result = modules().iter().fold(0.0, |total, module| total + required_fuel(*module));
    println!("Pt.1 You will need, {}", result);

    let result_pt_2 = modules()
        .iter()
        .fold(0.0, |total, module| total + total_fuel_for_module(*module));

    println!("Pt.2 You will need, {}", result_pt_2);

    let result_pt_2_rec = modules()
        .iter()
        .fold(0.0, |total, module| total + total_fuel_for_module_rec(*module));

    println!("Pt.2(recursive) You will need, {}", result_pt_2_rec);

}

// Calculates fuel cost for a module
fn required_fuel(x: f32) -> f32 {
    return (x / 3.0).floor() - 2.0;
}

// Calculates fuel needed for a module and for all fuel required to
// lift the fuel.
fn total_fuel_for_module(mass: f32) -> f32 {
    Fuel::new(mass).sum()
}

fn total_fuel_for_module_rec(mass: f32) -> f32 {
    step(0.0, required_fuel(mass))
}

fn step(total: f32, fuel: f32) -> f32 {
    if fuel > 0.0 {
        step(total + fuel, required_fuel(fuel))
    } else {
        total
    }
}

struct Fuel {
    fuel: f32,
}

impl Fuel {
    fn new(module_mass: f32) -> Fuel {
        Fuel { fuel: required_fuel(module_mass) }
    }
}

impl Iterator for Fuel {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.fuel >= 1.0 {
            let orig_fuel = self.fuel;
            self.fuel = required_fuel(orig_fuel);
            Some(orig_fuel)
        } else {
            None
        }
    }
}

fn modules() -> Vec<f32> {
    return vec![
        129315.0,
        138428.0,
        85143.0,
        119378.0,
        106438.0,
        136138.0,
        126273.0,
        61726.0,
        117121.0,
        107510.0,
        116139.0,
        137089.0,
        62862.0,
        89101.0,
        91623.0,
        121912.0,
        113802.0,
        68527.0,
        106791.0,
        71526.0,
        80210.0,
        140968.0,
        116768.0,
        114069.0,
        74451.0,
        72109.0,
        89284.0,
        65098.0,
        76986.0,
        52739.0,
        106469.0,
        112964.0,
        133216.0,
        110269.0,
        70285.0,
        52893.0,
        134567.0,
        70332.0,
        51686.0,
        116308.0,
        132269.0,
        101578.0,
        69560.0,
        137966.0,
        108829.0,
        94394.0,
        64614.0,
        77959.0,
        86005.0,
        112014.0,
        54597.0,
        108355.0,
        82805.0,
        54025.0,
        50093.0,
        139350.0,
        89057.0,
        108119.0,
        149167.0,
        90273.0,
        83649.0,
        58058.0,
        59560.0,
        63756.0,
        78767.0,
        112689.0,
        59109.0,
        103073.0,
        97051.0,
        122663.0,
        59326.0,
        63315.0,
        105423.0,
        134811.0,
        89578.0,
        105967.0,
        112749.0,
        77245.0,
        146275.0,
        97078.0,
        146862.0,
        75927.0,
        124553.0,
        103857.0,
        125861.0,
        131980.0,
        60928.0,
        109846.0,
        128001.0,
        71441.0,
        101655.0,
        110244.0,
        100550.0,
        149770.0,
        80374.0,
        76230.0,
        70359.0,
        113471.0,
        143101.0,
        148859.0
    ]
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_required_fuel() {
        assert_eq!(required_fuel(12_f32), 2_f32);
        assert_eq!(required_fuel(14_f32), 2_f32);
        assert_eq!(required_fuel(1969_f32), 654_f32);
        assert_eq!(required_fuel(100756_f32), 33583_f32);
    }

    #[test]
    fn test_total_fuel() {
        assert_eq!(total_fuel_for_module(12_f32), 2_f32);
        assert_eq!(total_fuel_for_module(1969_f32), 966_f32);
        assert_eq!(total_fuel_for_module(100756_f32), 50346_f32);
    }

    #[test]
    fn test_total_fuel_recursive() {
        assert_eq!(total_fuel_for_module_rec(12_f32), 2_f32);
        assert_eq!(total_fuel_for_module_rec(1969_f32), 966_f32);
        assert_eq!(total_fuel_for_module_rec(100756_f32), 50346_f32);
    }
}
