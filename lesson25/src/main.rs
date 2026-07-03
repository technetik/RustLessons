 struct Coffee {
    price: f64, 
    name: String, 
    is_hot: bool,
}

fn main() {
    let coffee: Coffee = make_coffee(String::from("Latte"), 4.99, true);
    println!("{},{},{}", coffee.name, coffee.price, coffee.is_hot);
    println!("-------");

    let coffee2: Coffee = make_coffee_2(String::from("Американо"), 8.99, true);
    println!("{},{},{}", coffee2.name, coffee2.price, coffee2.is_hot);
    println!("-------");

    let name: String = String::from("Latte1");
    let price: f64 = 3.99;
    let is_hot: bool = false;
    let latte =  Coffee {
        name, 
        price, 
        is_hot,
    };
    println!("{},{},{}", latte.name, latte.price, latte.is_hot);
}

// создание экземпляра структуры при помощи функцией
fn make_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
    Coffee {
        name: name, 
        price: price, 
        is_hot: is_hot, 
    } 
}

// альтернативный сокращенный синтасис для создания структуры при помощи
fn make_coffee_2(name: String, price: f64, is_hot: bool) -> Coffee {
    Coffee {
        name, 
        price, 
        is_hot, 
    } 
}
