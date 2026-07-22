struct Users {
    name: String,
    year: u32,
    height: u32,
}

// Связанные функции это функции привязанные к типу.
// Мы часто используем связанныке функции для конструкторов, например ::new()
impl Users{
    fn new(name: String, year: u32, height: u32) -> Self{
        Self { name, year, height }
    }

    /*
    // альтернативный синтаксис
    fn new(name: String, year: u32, height: u32) -> User{
        User { name, year, height }
    }
    */

    fn print_user(&self){
        println!("Имя: {}", self.name);
        println!("Возраст: {}", self.calc_age()); // вызываем метод внутри метода
        println!("Год: {}", self.year);
        println!("Рост: {}", self.height);
    }

    fn calc_age(&self) -> u32{
        2026 - self.year
    }
}

fn main() {
    let user = Users::new(String::from("Петя"), 1999, 188);
    user.print_user();
}
