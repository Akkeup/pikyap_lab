#![allow(unused)]

use std::io; // библиотека ввода/вывода
use colored::*; // цвет вывода текста в консоли toml - colored = "2.1"

fn main() {
    loop {
        let mut coef = String::new();
        let mut result: Vec<f64> = Vec::new();

        println!("Введите коэффициенты биквадратного уравнения:");

        io::stdin().read_line(&mut coef).expect("Ошибка при чтении строки");

        let nums_result: Vec<Result<f64, _>> = coef
            .trim().
            split_whitespace()
            .map(|x| x.parse())
            .collect();

        if nums_result.len() < 3 {
            println!("{}", "Необходимо ввести 3 коэффициента".red());
            continue;
        }

        if nums_result.iter().any(|x| x.is_err()) {
            println!("{}", "Один или несколько параметров не являются дробным числом -> f64".red());
            continue;
        }

        let nums: Vec<f64> = nums_result
            .into_iter()
            .map(|x| x.unwrap())
            .collect();

        if nums[0] == 0.0 {
            println!("{}", "Коэффициент a не может быть равен нулю".red());
            continue;
        }

        let a = nums[0];
        let b = nums[1];
        let c = nums[2];

        let discriminant = b * b - 4.0 * a * c;
        if discriminant >= 0.0 {
            let t1 = (-b + discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b - discriminant.sqrt()) / (2.0 * a);

            if t1 >= 0.0 {
                result.push(t1.sqrt());
                result.push(-t1.sqrt());
            }

            if t2 >= 0.0 {
                result.push(t2.sqrt());
                result.push(-t2.sqrt());
            }

            if t1 != t2 && t2 >= 0.0 {
                result.push(t2.sqrt());
                result.push(-t2.sqrt());
            }

            for i in result {
                if i == 0.0 || i == -0.0 {
                    continue;
                }
                println!("{}", format!("Корень: {i}").green());
            }
            break;
        } else {
            println!("{}", "Действительных корней нет".red());
        }
    }
}