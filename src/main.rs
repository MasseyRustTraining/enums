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

// This is zero-overhead because niche optimization.
// 8 bytes on 64-bit machine.
pub enum RR<'a> {
    Ref64(&'a u64),
    Ref32(&'a u32),
}

#[derive(Debug, Clone, Copy)]
pub struct Dud;

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Rgb([u8; 3]),
    Rgba([u8; 4]),
    Monochrome(bool),
    Gray { scale: u8, alpha: u8, dud: Dud },
}
use Color::*;

pub static SBLACK: Color = Rgb([0, 0, 0]);
pub const CBLACK: Color = Rgb([0, 0, 0]);

impl Color {
    fn show_color(self) {
        fn print_components(components: &[u8]) {
            for c in components {
                print!("{:02x}", c);
            }
            println!();
        }

        let s = 5;
        let a = 7;
        match self {
            Rgb(components) => print_components(&components),
            Rgba(components) => print_components(&components),
            Monochrome(bw) => if bw {
                println!("ffffff")
            } else {
                println!("000000")
            },
            Gray { scale: 0, alpha: 0, .. } => println!("transparent"),
            Gray { scale: s0, alpha: a0, .. } if s == s0 && a == a0 => {
                println!("special");
            }
            Gray { scale: s, alpha: a, .. } => {
                let rgba = [s, s, s, a];
                print_components(&rgba);
            }
        }
    }
}

pub fn tuple_fn((_x, _y): (u32, u32)) {
}

fn main() {
    let c = Color::Rgb([17, 0, 255]);
    c.show_color();

    let c = Color::Gray { scale: 0, alpha: 0, dud: Dud };
    c.show_color();
}
