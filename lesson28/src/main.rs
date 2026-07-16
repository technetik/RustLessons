#[derive(Debug)]
struct Users {
    name: String,
    age: u32,
    year: u32,
}

impl Users{
    fn print_user(self){
        println!("Имя: {}", self.name);
        println!("Возраст: {}", self.age);
        println!("Год: {}", self.year);
    }
    fn year_plus(mut self){
        self.year = self.year + 1;
        println!("{:#?}", self);
    }
}

fn main() {



}
