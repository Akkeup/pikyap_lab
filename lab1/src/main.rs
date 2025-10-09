#![allow(unused)] // игнорируем неиспользуемые переменные/методы и т.д.

use std::io; // обычная библиотека ввода/вывода
use colored::*; // цвет вывода текста в консоли toml - colored = "2.1"

fn main() {
    loop {
        let mut coef = String::new();
        let mut result: Vec<f64> = Vec::new();

        println!("Введите коэффициенты биквадратного уравнения:");

        io::stdin().read_line(&mut coef).expect("Ошибка при чтении строки");

        let nums: Vec<f64> = coef
            .trim()
            .split_whitespace()
            .map(|x| x.parse().expect("Введите корректное число"))
            .collect();

        if nums.len() == 3 && nums[0] != 0.0 {
            let a = nums[0];


            let b = nums[1];
            let c = nums[2];

            let discriminant = b*b - 4.0*a*c;
            if discriminant >= 0.0 {
                let x1 = (-b + discriminant.sqrt()) / (2.0 * a);
                let x2 = (-b - discriminant.sqrt()) / (2.0 * a);
                result.push(x1);
                result.push(x2);
        
                for i in result {
                    println!("{}", format!("Ваши корни: {i}").green());
                }

                break;
            } 
            else {
                println!("{}", "Нет корней".red());
            }
        } 
        else if nums[0] == 0.0 {
            println!("{}", "a - не может быть нулевым".red());
        } 
        else {
            println!("{}", "Введите 3 коэффициента".red());
        }
    }
}
