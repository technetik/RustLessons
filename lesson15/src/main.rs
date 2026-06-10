// Recursion
fn countup(sec: i32) {
    if sec == 0 {
        println!("Стоп");
    } else {
        println!("{sec} обратный отсчет ");
        countup(sec-1);
    }

}
fn main() {
    countup(8);
}
