// Recursion
fn countup(sec: i32) {
    // base case условие которое останавливает рекурсию
    if sec == 0 {
        println!("Стоп рекурсия"); 
    } else {
        println!("{sec} обратный отсчет ");
        countup(sec-1);
    }

}
fn main() {
    countup(8);
}
