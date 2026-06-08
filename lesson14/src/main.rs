fn main() {
    // Конструкция match в Rust — это мощный аналог switch-case
    // Пример 1
    let number = 2;

    match number {
        1 => println!("Один"),
        2 => println!("Два"),
        3 => println!("Три"),
        _ => println!("Что-то другое"), // Ветка по умолчанию
    }
    // Пример 2
    let number1 = true;
    let value = match number1 {
        true => 20,
        false => 40,
    };
    println!("{value}");

    // Пример 3
    // Объединение условий (|) и диапазоны (..=)
    let age = 18;

    match age {
        0..=12 => println!("Ребенок"),
        13..=19 => println!("Подросток"),
        20 | 21 => println!("Совершеннолетие в США"),
        _ => println!("Взрослый"),
    }

    // Пример 4
    // Связывание переменных через @
    // Если нужно проверить диапазон, но одновременно сохранить само значение в переменную:
    let age = 15;

    match age {
        age_val @ 13..=19 => println!("Подростку {} лет", age_val),
        _ => println!("Другой возраст"),
    }



}
