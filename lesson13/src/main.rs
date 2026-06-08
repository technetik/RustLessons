// аналог тернарного оператора
// тернарного оператора в rust нет
fn even_or_odd(number: i32){
    let result: _= if number % 2 == 0 { "четное" } else { "нечетное" };
    println!("Это число {result}");
}

fn main() {
    // Пример 1
    let number = 10;
    // Полный аналог: let is_even = (number % 2 == 0) ? "четное" : "нечетное";
    let is_even = if number % 2 == 0 { "четное" } else { "нечетное" };
    println!("{}", is_even); // Выведет: четное


    // Пример 2
    even_or_odd(17);
    even_or_odd(100);
}
