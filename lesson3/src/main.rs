fn main() {
    // Область видимости Scopes

    // Пример 1
    let price = 100;
    {
        let price = 400;
        println!("Price {price}"); // 400
    }
    println!("Price {price}");  // 100

    // Пример 2
    let y = 2023;
    let y = y + 1;
    // внутри блока своя область видимости
    {
        let y = y - 24;
        println!("{y}"); // 2000
    }
    println!("{y}"); // 2024

    // Пример 3
    // внутри блока можно проводить вычисления и возвращать значение
    let num1 = 50;
    let calc = {
        let num2 = 5 + 15;
        num1 * num2
    };
    println!("{calc}")

}
