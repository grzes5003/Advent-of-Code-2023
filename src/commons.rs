
pub fn lcm<T>(vec: Vec<T>) -> T
    where T: PartialEq + std::ops::Rem<Output = T> + Default + Copy + std::ops::Mul<Output = T> + std::ops::Div<Output = T> {
    let mut lcm = vec[0];
    for i in vec.iter().skip(1) {
        lcm = lcm * *i / gcd(lcm, *i);
    }
    lcm
}

pub fn gcd<T>(mut a: T, mut b: T) -> T
    where T: PartialEq + std::ops::Rem<Output = T> + Default + Copy
{
    while b != T::default() {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}