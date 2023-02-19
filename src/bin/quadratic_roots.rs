fn main() {
    print_quadratic_roots(1.0, -3.0, 2.0); // 1 and 2
    print_quadratic_roots(4.0, 2.0, 2.0); // complex
    print_quadratic_roots(2.0, 2.0, 0.5); // -0.5
}

// Exercise: Write a program that prints the roots of a quadratic equation (given hardcoded
// coefficients a, b and c).
/// a x^2 + b x + c
fn print_quadratic_roots(a: f64, b: f64, c: f64) {
    let discriminant = b.powi(2) - 4.0 * a * c;
    // is there a nicer way to say np.close?
    if discriminant.abs() < 1e-10 {
        let root = -b / (2.0 * a);
        println!("Both roots are: {root}");
    } else if discriminant > 0.0 {
        let roots = [-1.0, 1.0].map(|sign| (-b + sign * discriminant.sqrt()) / (2.0 * a));
        println!("First root: {}, second root {}", roots[0], roots[1]);
    } else {
        println!("I'll skip on complex numbers");
    }
}
