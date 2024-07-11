// use std::fs;
// fn main() {
//     let file = fs::metadata("./file.txt");
//     let file = file.unwrap();
//     println!("Hello, world! {}", file.file_type());
// }

use std::env;
use std::process::{Command, Output};
use cursive::{Cursive, CursiveExt};
use cursive::views::{Dialog, TextView, SelectView, LinearLayout};
fn main() {
    let env_data: Vec<String> = env::args().collect();
    let a_folder : String;

    a_folder = if env_data.len() > 1 {env_data[1].clone()} else {".".to_string()};
    println!("{}", a_folder);

    let mut cursive_instance = Cursive::new();
    cursive_instance.add_global_callback('q', |c| c.quit());
    cursive_instance.add_layer(Dialog::text(""));
    cursive_instance.add_layer(TextView::new("Press q to quit"));

    something(&mut cursive_instance, &a_folder);

    cursive_instance.run();
}

fn something(cursive: &mut Cursive, path: &String)
{
    let output: std::process::Output = Command::new("pwd").output().expect("Something wrong with pwd command");
    let mut current_path = String::from_utf8(output.stdout).expect("Something wrong with current path from pwd command");
    current_path.pop();
    env::set_current_dir(path.clone()).expect("Something wrong when set current dir from pwd command");
    
    cursive.pop_layer();
    let     select_view: SelectView = SelectView::<String>::new();
    let mut select_view: SelectView = select_view.on_submit(|c: &mut Cursive, i: &String| something(c, i));
    select_view.add_item("Back", "..".to_string());

    let list = match get_dir_list() {
        Some(item) => item,
        None => unreachable!()
    };

    if !list.is_empty() {
        for item in list{
            select_view.add_item(item.clone(), item);
        }
    }

    cursive.add_layer(
        Dialog::around(
            LinearLayout::horizontal().child(select_view)
        ).title("FOLDER")
    );

}

fn get_dir_list() -> Option<Vec<String>>{
    let list: Output = Command::new("ls").output().expect("Something wrong with ls command");
    let list: String = String::from_utf8(list.stdout).expect("Something wrong with file and folder");
    let list: Vec<String> = list.trim().split("\n").map(|i| i.to_string()).collect();
    return Some(list);
}