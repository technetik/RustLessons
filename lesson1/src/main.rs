fn main() {
    let num1 = 40;
    let num2 = 25;
    let num3 = num1 + num2;


    println!("Hello, world! num1 = {} ", num1); 
    // можно использовать математические операции
    println!("Hello, world! num1 = {} ", num1 + 5); 
    // c Rust 1.58 появиля альтернативный синтаксис для println!()  
    println!("Hello, world! num1 = {num1} ");    

    // можно выводить несколько аргументов
    println!("Hello, world! num1 = {} num2 = {} num3 = {}", num1, num2, num3); 
    // можно ссылаться на аргументы по номеру {0} {1} и т д
    println!("Hello, world! num1 = {0} num2 = {1} ", num1, num2);
    // так же можно вызывать их несколько раз, в произвольном порядке
    println!("Hello, {1} world! num1 = {0} num2 = {1} and {0}", num1, num2);
    
}
