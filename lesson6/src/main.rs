fn main() {
    let is_test1 = true;
    let is_test2 = false;
    println!("Тест 1 = {is_test1} Тест 2 = {is_test2}");  // true false


    let meters: i32 = 100;
    let is_daleko = meters > 110; // false
    println!("{is_daleko}");  // false
    println!("{}", meters);
    println!("{}", meters.is_positive()); // true. метод проверяет что число положительное
    println!("{}", meters.is_negative()); // false метод проверяет что число отрицательное

    // Логическая инверсия с !
    // Инвертируем логическое значения используя !
    println!("{}", !true);
    println!("{}", !false);

    let age = 13;
    let is_pasport = age >= 14; // false
    let no_is_pasport = !is_pasport; // true
    println!(" Тебе {age} и поэтому паспорт {is_pasport}, дадут позже {no_is_pasport}"); // 13 true false



}
