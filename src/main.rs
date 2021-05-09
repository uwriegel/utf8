use std::convert::TryInto;

//use std::iter::FromIterator;

fn main() {
    let fire = "fire";
    println!("Name: {}", fire[0..2].to_string());

    let fi_re = "ﬁre";
    let s = fi_re.get(0..2);
    println!("Name: {}", fi_re[0..2].to_string());

    let name = "Jürgen";
    //println!("Name: {}", name[0..2].to_string());

    let smiley = "👌✊👏";
    //println!("Name: {}", smiley[0..2].to_string());

    let teil: String = name.chars().into_iter().take(2).collect();
    println!("Name: {}", teil);

    let teil: String = name.chars().into_iter().skip(4).take(2).collect();
    println!("Name: {}", teil);

    let teil: String = fi_re.chars().into_iter().take(2).collect();
    println!("Name: {}", teil);

    let teil: String = smiley.chars().into_iter().take(2).collect();
    println!("Name: {}", teil);

    print_lens(fire);
    print_lens(fi_re);
    print_lens(name);
    print_lens(smiley);

    fn print_lens(text: &str) {
        println!("Länge von {}, bytes: {}, chars: {}", text, text.len(), text.chars().count());
    }
}
