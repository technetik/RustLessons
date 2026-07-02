fn main() {
    // Шаблон структуры
    struct Coffee {
        price: f64, 
        name: String, 
        is_hot: bool,
    }

    // Создаем экземпляр структуры
    let mocha = Coffee {
        name: String::from("Mocha"),
        price: 4.99,
        is_hot: false,  
    };

    println!("{},{},{}", mocha.name, mocha.price, mocha.is_hot);

    // Создаем изменяемый экземпляр структуры
    let mut robusta = Coffee {
        name: String::from("Robusta"),
        price: 11.99,
        is_hot: false,  
    };
    
    println!("{},{},{}", robusta.name, robusta.price, robusta.is_hot);

    robusta.name = String::from("Robusta One");
    robusta.price = 100.99;
    println!("{},{},{}", robusta.name, robusta.price, robusta.is_hot);
}
