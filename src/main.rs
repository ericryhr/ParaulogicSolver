use std::{fs::File, io::{Read, stdin}};

//Amb els unwraps ens saltem &el tractament d'errors
fn main() {
    println!("OBRINT DICCIONARI");
    let mut file = File::open("media/diccionari.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let diccionari = contents.split("\n");
    println!("DICCIONARI LLEGIT");

    let mut lletra_obligatoria = String::new();
    let mut altres_lletres = String::new();
    
    println!("LLETRA OBLIGATORIA EN MAJUSCULES");
    stdin().read_line(&mut lletra_obligatoria)
        .ok()
        .expect("Error: Torni a escruire la lletra");

    println!("ALTRES LLETRES EN MAJUSCULES");
    stdin().read_line(&mut altres_lletres)
        .ok()
        .expect("Error: Torni a escruire les lletres");

    println!("CREANT ABECEDARI");
    //Codi merda l'odio molt
    let mut lletres = ['A'; 7];
    lletres[0] = lletra_obligatoria.chars().nth(0).unwrap();
    for i in 0..6 {
        lletres[i+1] = altres_lletres.chars().nth(i).unwrap();
    }

    println!("ABECEDARI {:?}", lletres);

    println!("COMPROVANT DICCIONARI");
    'bucle_paraules: for paraula in diccionari {
        if paraula.len() < 3 { continue; }
        let mut found_obligatoria = false;
        for c in paraula.chars() {
            let mut found_lletra = false;
            if c == lletres[0] {found_obligatoria = true; }
            else {
                for i in 1..7 {
                    if c == lletres[i] {found_lletra = true; break; }
                }
                if !found_lletra { continue 'bucle_paraules; }
            }
        }
        if found_obligatoria { println!("- {}", paraula);}
    }

}
