// fn print_elements(elements: &Vec<String>) {
//     for element in elements {
//         println!("{}", element);
//     }
// }

fn print_elements(elements: &[String]) {
    elements
        .iter()
        .map(|el| format!("{} {}", el, el))
        .for_each(|el| println!("{}", el));
}

fn shorten_strings(elements: &mut [String]) {
    elements.iter_mut().for_each(|el| el.truncate(1));
}

fn to_uppercase(elements: &[String]) -> Vec<String> {
    elements
        .iter()
        .map(|el| el.to_uppercase())
        .collect::<Vec<String>>()
}

fn main() {
    let mut colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];

    // print_elements(&colors[1..3]);

    // shorten_strings(&mut colors);
    // println!("{:#?}", colors);

    let uppercased_colors = to_uppercase(&colors);
    println!("{:#?}", uppercased_colors);
}
