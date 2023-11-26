use std::io;
use rand::Rng;

fn random_sort(){
    let _chifoumi_list: [&str; 3] = ["Pierre","Feuille","Ciseaux"];
    let mut rng = rand::thread_rng();
    let rdm_int = rng.gen_range(1..=3);
    if let Some(rdm_choice) = _chifoumi_list.get(rdm_int){
    print!(" Le robot joue {rdm_choice} ! ");
    }
}

fn main() {
    let mut user_str = String::new();
    println!("Pierre, Feuille, Ciseaux ?");
    io::stdin()
    .read_line(&mut user_str)
    .expect("err");
    //let user_str = user_choice.to_lowercase();  
    if user_str.contains("pierre") || user_str.contains("feuille") || user_str.contains("ciseaux"){
        println!("Vous jouez {user_str}");
        random_sort();

    } else {
        print!("valeur incorrecte : {user_str}");
    }
        

    

    
}
