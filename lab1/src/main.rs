#![allow(unused)] // игнорируем неиспользуемые переменные/методы и т.д.

use std::io; // обычная библиотека ввода/вывода
use colored::*; // цвет вывода текста в консоли toml - colored = "2.1"

fn main() {
    loop {
        let mut coef = String::new();
        let mut result: Vec<f64> = Vec::new();

        println!("Введите коэффициенты биквадратного уравнения:");

        io::stdin().read_line(&mut coef).expect("Ошибка при чтении строки");

        let nums_result: Vec<Result<f64, _>> = coef
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<f64>())
            .collect();

        if nums_result.iter().any(|x| x.is_err()) {
            println!("{}", "Один или несколько параметров не являются дробным числом -> f64".red());
            continue; // запрашиваем ввод заново
        }

        let nums: Vec<f64> = coef
            .trim()
            .split_whitespace()
            .map(|x| x.parse().expect("Введите корректное число"))
            .collect();

        if nums.len() == 3 && nums[0] != 0.0 {
            let a = nums[0];
            let b = nums[1];
            let c = nums[2];

            let d = b*b - 4.0*a*c;
            if d >= 0.0 {
                let t1 = (-b + d.sqrt()) / (2.0 * a);
                let t2 = (-b - d.sqrt()) / (2.0 * a);

                if t1 >= 0.0 {
                    result.push(t1.sqrt());
                    result.push(-t1.sqrt());
                }

                if t2 >= 0.0 && t2 != t1 {
                    result.push(t2.sqrt());
                    result.push(-t2.sqrt());
                }
                else {
                    for i in result {
                        println!("{}", format!("Корень: {i}").green());
                    }
                    break;
                }
            } 
            else {
                println!("{}", "Действительных корней нет".red());
            }
        } 
        else if nums[0] == 0.0 {
            println!("{}", "a - не может быть нулевым".red());
        } 
        else if nums[0] != 2.0 {
            println!("{}", "один из параметров не конвертируется в дробное значение".red());
        }
        else {
            println!("{}", "Введите 3 коэффициента".red());
        }
    }
}
