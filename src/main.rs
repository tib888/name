use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashSet;

fn main() -> std::io::Result<()> {   
    let f = File::open(r#"words.txt"#)?;
    let reader = BufReader::new(f);

    let mut words = HashSet::<[u8;5]>::new();

    for line in reader.lines() {
        let word = line.unwrap();
        if word.len() == 5 || word.len() == 4 {
            let c = word.as_bytes();
            let w = [ c[0], c[1], c[2], c[3], if word.len() == 4 { ' ' as u8 } else { c[4] } ];
            words.insert(w); 
        }
    }           

    //println!("{}", words.len());

    let letters = b" ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let b = b"AEIOUY";
    
    for l0 in 1..letters.len() {
        for l1 in 1..letters.len() {
            if l0 == l1 {
                continue;
            }

            for l2 in 1..letters.len() {
                if l1 == l2 {
                    continue;
                }
                
                for l3 in 1..letters.len() {                    
                    if l2 == l3 {
                        continue;
                    }

                    for l4 in 0..letters.len() {
                        if l3 == l4 {
                            continue;
                        }

                        let mut v = 0;
                        if b.contains(&letters[l0]) { v += 1; }
                        if b.contains(&letters[l1]) { v += 1; }
                        if b.contains(&letters[l2]) { v += 1; }
                        if b.contains(&letters[l3]) { v += 1; }
                        if b.contains(&letters[l4]) { v += 1; }

                        if v < 2 || v > 3 { 
                            continue; 
                        }

                        let name = [ letters[l0], letters[l1], letters[l2], letters[l3], letters[l4] ];
                        if !words.contains(&name[0..5]) {
                            println!("{}{}{}{}{}", letters[l0] as char, letters[l1] as char, letters[l2] as char, letters[l3] as char, letters[l4] as char);
                        }
                    }
                }   
            }        
        }
    }

    Ok(())   
}
