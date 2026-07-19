#[derive(Debug)]
struct Users {
    name: String,
    year: u32,
    height: u32,
}

impl Users{
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
    let user1 = Users{
        name: String::from("Петя"),
        year: 1989,
        height: 188,
    };

    user1.print_user();
}
