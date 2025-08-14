


pub fn screen_clear(){
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

pub fn wait_for_enter()-> (){
    std::io::stdin().read_line(&mut String::new()).unwrap();
}