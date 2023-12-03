use std::fs;

fn main() {
    let puzzle: String = fs::read_to_string("./puzzle")
        .expect("The file named 'puzzle' should be in the current working directory!");

    let mut numbers :Vec<Number> = Vec::new();
    let mut symbols :Vec<Symbol> = Vec::new();

    let lines :Vec<&str> = puzzle.split("\n").collect();
    for y in 0..lines.len() {
        if lines[y].trim() == "" { continue };

        let chars :Vec<char> = lines[y].chars().collect();

        let mut current_number = Number {
            value: 0,
            coordinates: Vec::new()
        };

        for x in 0..chars.len() {
            if chars[x].is_numeric() {
                if current_number.value == 0 {
                    current_number.value = chars[x].to_digit(10).unwrap(); 
                } else {
                    current_number.value = format!("{}{}", current_number.value, chars[x].to_digit(10).unwrap()).parse().unwrap();
                }
                current_number.coordinates.push(Coordinate {x,y});
            } else {
                if current_number.value > 0 {
                    numbers.push(current_number.clone());
                    current_number.value = 0;
                    current_number.coordinates = Vec::new();
                }

                if chars[x] != '.' {
                    symbols.push(Symbol {
                        value: chars[x], 
                        coordinate: Coordinate {x,y} 
                    });
                }
            }
        }
    }


    let mut result = 0;
    for number in numbers {
        println!("");
        println!("Number {}", number.value);
        println!("Coordinates: ");

        let mut adjecent_symbol :char = '.';

        for coordinate in number.coordinates {
            println!("  x: {}, y: {}", coordinate.x, coordinate.y);
            for symbol in symbols.iter() {
                if euclidian_distance(&symbol.coordinate, &coordinate) < 2.0 {
                    adjecent_symbol = symbol.value;
                    break;
                }
            }
        }

        if adjecent_symbol != '.' {
            result += number.value;
            println!("Has adjecent symbol: {}", adjecent_symbol);
            adjecent_symbol = '.';
        }
    }

    println!("The sum of all the numbers is: {}", result);
}

fn euclidian_distance(coord1: &Coordinate, coord2: &Coordinate) -> f64 {
    let x_squared = (coord2.x as f64 - coord1.x as f64).powi(2);
    let y_squared = (coord2.y as f64 - coord1.y as f64).powi(2);
    (x_squared + y_squared).sqrt()
}

// A number has a value and a set of coordinates
// These coordinates can be used to easily check if it is near a symbol
#[derive(Clone)]
struct Number {
    value: u32,
    coordinates: Vec<Coordinate>
}

struct Symbol {
    value: char,
    coordinate: Coordinate
}

#[derive(Clone)]
struct Coordinate {
    x: usize,
    y: usize
}
