mod structs;
use std::io;

use structs::point::*;
fn main() {
    println!("digite o valor de x:");
    let mut value: String = String::new();

    io::stdin()
        .read_line(&mut value)
        .expect("erro ao obter valor");

    let x_ini: i32 = value.trim().parse::<i32>().unwrap();

    let mut point = Point::new(x_ini, 0);

    point.y = Point::calc_function(&point, first_degree_function);
    Point::show_coordenate(&point);
    point.y=Point::calc_function(&point, second_degree_function);
    Point::show_coordenate(&point);

}

fn first_degree_function(x: i32) -> i32 {
    println!("função de primeiro grau:\n");

    println!("f(x)=(x*2)-(x+2)");
    println!("f({})={}", x, (x * 2) - (x + 2));
 
    (x * 2) - (x + 2)
}
fn second_degree_function(x: i32) -> i32 {
    println!("função de segundo grau:\n");

    println!("f(x)= (x*x) + (x/2)");
    println!("f({})={}", x, (x * x) + (x / 2));
  
    (x * x) + (x / 2)
}
