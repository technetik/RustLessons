// Может принимать строку &String и срез строки &str
// В Rust для типа String реализован трейт Deref<Target = str>. 
// Когда компилятор видит, что функция ждет &str, а вы даете &String, 
// он автоматически преобразует превращает &String в &str
fn txt_print(txt: &str) { // &String, &str 
    println! ("{txt} prints");
}

fn main() {
    let txt1: String = String:: from ("Текс 1");
    txt_print(&txt1);
    let txt2: &str = "Текс 2";
    txt_print(&txt2);
}
