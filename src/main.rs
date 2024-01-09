/*
// "Product type": 65536 possible values.
struct S {
    x: u8,
    y: u8,
}

// "Sum type": 512 possible values.
enum S {
    X(u8),
    Y(u8),
}
*/

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Color {
    Rgb([u8; 3]),
    Rgba([u8; 4]),
    Monochrome(bool),
    Gray { scale: u8, alpha: u8 },
}
use Color::*;

impl Color {
    fn show_color(self) {
        fn print_components(components: &[u8]) {
            for c in components {
                print!("{:02x}", c);
            }
            println!();
        }

        match self {
            Rgb(components) => print_components(&components),
            Rgba(components) => print_components(&components),
            Monochrome(bw) => if bw {
                println!("ffffff")
            } else {
                println!("000000")
            },
            Gray { scale: 0, alpha: 0 } => println!("transparent"),
            Gray { scale: s, alpha: a } => {
                let rgba = [s, s, s, a];
                print_components(&rgba);
            }
        }
    }
}

fn main() {

    let c = Color::Rgb([17, 0, 255]);
    c.show_color();

    let c = Color::Gray { scale: 0, alpha: 0 };
    c.show_color();
}
