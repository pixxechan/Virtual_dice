use rand::Rng;
use std::env;

fn roll_die(sides: u32) -> u32 {
    rand::thread_rng().gen_range(1..=sides)
}

fn print_d6(n: u32) {
    let faces = [
        ["   ", " ● ", "   "],  // 1
        ["●  ", "   ", "  ●"],  // 2  
        ["●  ", " ● ", "  ●"],  // 3
        ["● ●", "   ", "● ●"],  // 4
        ["● ●", " ● ", "● ●"],  // 5
        ["● ●", "● ●", "● ●"],  // 6
    ];
    
    if n >= 1 && n <= 6 {
        println!("┌───┐");
        for row in &faces[(n-1) as usize] {
            println!("│{}│", row);
        }
        println!("└───┘");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let sides = if args.len() > 1 {
        args[1].parse().unwrap_or(6)
    } else {
        6
    };
    
    let result = roll_die(sides);
    
    if sides == 6 {
        print_d6(result);
    } else {
        println!("🎲 d{}: {}", sides, result);
    }
}

// Cargo.toml:
// [dependencies]
// rand = "0.8"
