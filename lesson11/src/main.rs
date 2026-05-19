use std::io; // подключаем библиотеку ввода вывода

fn main() {
    let mut user_data = String::new(); // создаем пустую  строку
    
    println!("Напечатай число");
    io::stdin().read_line(&mut  user_data).expect("Не удалось прочитать строку");

    let number: usize = match  user_data.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Ошибка: введено не число!");
            return;
        }
    };

    println!("Вы ввели число: {}", number);


    // Треугольник №1 (цикл 1) 
    let mut word1 = String::new();
    let mut a = 0;

    while  number > a {
        word1.push_str("#");     
        println!("{}", word1);
        a = a + 1;
    }
    println!("\n");
    

    // Треугольник №2 (цикл 2)
    //let mut word1 = String::new();
    let mut a = number;

    while  a > 0 {
        let word1: String = "#".repeat(a);
        //word1.push_str("#");     
        println!("{}", word1);
        a = a - 1;
    }

    println!("\n");

    // Треугольник №3 (цикл 3)
    let mut a = 1;
    while  a <= number {
        //let probel: String = " ".repeat(number - 1.try_into().unwrap());
        let probel: String = " ".repeat(number - a);
        let word1: String = "#".repeat(a);
        println!("{}{}",probel, word1);
        a = a + 1;
    }

    println!("\n");

    // Треугольник №4 (цикл 4)
    let mut a = 0;
    while  a <= number {
        let probel: String = " ".repeat(a);
        let word1: String = "#".repeat(number - a);
        println!("{}{}",probel, word1);
        a = a + 1;
    }  

    println!("\n");


}


