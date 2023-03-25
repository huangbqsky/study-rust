use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct RGB {
    r: u8,
    g: u8,
    b: u8,
}
impl std::str::FromStr for RGB {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r: u8 = u8::from_str_radix(&s[1..3], 16)?;
        let g: u8 = u8::from_str_radix(&s[3..5], 16)?;
        let b: u8 = u8::from_str_radix(&s[5..7], 16)?;
        Ok(RGB{r, g, b})
    }
}
fn main() { 
    let code: &str = &r"#fa7268";
    match RGB::from_str(code) {
        Ok(rgb) => println!("The RGB color code is : R:{} G:{} B:{}", rgb.r, rgb.g, rgb.b),
        Err(_) => println!("{} is not a valid color hex code!", code),  
    }
}