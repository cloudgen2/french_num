extern crate regex;
use std::io::{self, Write};
use rand::Rng;
use regex::Regex;

fn conversion(s: &mut String, number: u32) {
    let diff: u32;
    if number<1000 {
        below1000(s, number);
    } else if number == 1000 {
        s.push_str("mille");
    } else if number < 2000 {
        s.push_str("mille-");
        diff = number - 1000;
        below1000(s, diff);
    } else if number == 2000 {
        s.push_str("deux-milles");
    } else if number < 3000 {
        s.push_str("deux-mille-");
        diff = number - 2000;
        below1000(s, diff);
    } else if number == 3000 {
        s.push_str("trois-milles");
    } else if number < 4000 {
        s.push_str("trois-mille-");
        diff = number - 3000;
        below1000(s, diff);
    } else if number == 4000 {
        s.push_str("quatre-milles");
    } else if number < 5000 {
        s.push_str("quatre-mille-");
        diff = number - 4000;
        below1000(s, diff);
    } else if number == 5000 {
        s.push_str("cinq-milles");
    } else if number < 6000 {
        s.push_str("cinq-mille-");
        diff = number - 5000; 
        below1000(s, diff);
    } else if number == 6000 {
        s.push_str("six-milles");
    } else if number < 7000 {
        s.push_str("six-mille-");
        diff = number - 6000;
        below1000(s, diff);
    } else if number == 7000 {
        s.push_str("sept-milles");
    } else if number < 8000 {
        s.push_str("sept-mille-");
        diff = number - 7000;
        below1000(s, diff);
    } else if number == 8000 {
        s.push_str("huit-milles");
    } else if number < 9000 {
        s.push_str("huit-mille-");
        diff = number - 8000;
        below1000(s,diff);
    } else if number == 9000 {
        s.push_str("neuf-milles");
    } else if number < 10000 {
        s.push_str("neuf-mille-");
        diff = number - 9000;
        below1000(s, diff);
    }
}
fn below1000(s: &mut String, number: u32) {
    let diff: u32;
    if number < 100 {
        below100(s, number);
    } else if number == 100 {
        s.push_str("cent");
    } else if number < 200 {
        s.push_str("cent-");
        diff = number - 100;
        below100(s, diff);
    } else if number == 200 {
        s.push_str("deux-cents");
    } else if number < 300 {
        s.push_str("deux-cent-");
        diff = number - 200;
        below100(s, diff);
    } else if number == 300 {
        s.push_str("trois-cents");
    } else if number < 400 {
        s.push_str("trois-cent-");
        diff = number - 300;
        below100(s, diff);
    } else if number == 400 {
        s.push_str("quatre-cents");
    } else if number < 500 {
        s.push_str("quatre-cent-");
        diff = number - 400;
        below100(s, diff);
    } else if number == 500 {
        s.push_str("cinq-cents");
    } else if number < 600 {
        s.push_str("cinq-cent-");
        diff = number - 500;
        below100(s, diff);
    } else if number == 600 {
        s.push_str("six-cents");
    } else if number < 700 {
        s.push_str("six-cent-");
        diff = number - 600;
        below100(s, diff);
    } else if number == 700 {
        s.push_str("sept-cents");
    } else if number < 800 {
        s.push_str("sept-cent-");
        diff = number - 700;
        below100(s, diff);
    } else if number == 800 {
        s.push_str("huit-cents");
    } else if number < 900 {
        s.push_str("huit-cent-");
        diff = number - 800;
        below100(s, diff);
    } else if number == 900 {
        s.push_str("neuf-cents");
    } else if number < 1000 {
        s.push_str("neuf-cent-");
        diff = number - 900;
        below100(s, diff);
    }
}

fn below100(s: &mut String, number: u32) {
    let numbers = vec!["zéro","un","deux","trois","quatre","cinq","six","sept","huit","neuf","dix","onze","douze","treize","quatorze","quinze","seize","dix-sept","dix-huit","dix-neuf","vingt"];
    let diff: u32;
    if number < 21 {
        s.push_str(numbers[number as usize]);
    } else if number < 30 {
        s.push_str(numbers[20]);
        diff = number - 20;
        if diff == 1 {
            s.push_str("-et-");
        } else {
            s.push_str("-");
        }
        s.push_str(numbers[diff as usize]);
    } else if number < 40 {
        s.push_str("trente");
        diff = number - 30;
        if diff > 0 {
            if diff == 1 {
                s.push_str("-et-");
            } else {
                s.push_str("-");
            }
            s.push_str(numbers[diff as usize]);
        }
    } else if number < 50 {
        s.push_str("quarante");
        diff = number - 40;
        if diff > 0 {
            if diff == 1 {
                s.push_str("-et-");
            } else {
                s.push_str("-");
            }
            s.push_str(numbers[diff as usize]);
        }
    } else if number < 60 {
        s.push_str("cinquante");
        diff = number - 50;
        if diff > 0 {
            if diff == 1 {
                s.push_str("-et-");
            } else {
                s.push_str("-");
            }
            s.push_str(numbers[diff as usize]);
        }
    } else if number < 80 {
        s.push_str("soixante");
        diff = number - 60;
        if diff > 0 {
            if diff == 1 || diff == 11 {
                s.push_str("-et-");
            } else {
                s.push_str("-");
            }
            s.push_str(numbers[diff as usize]);
        }
    } else if number == 80 {
        s.push_str("quatre-vingts");
    } else if number < 100 {
        s.push_str("quatre-vingt-");
        diff = number - 80;
        s.push_str(numbers[diff as usize]);
    } 
}

fn question(q: u32, min: u32, max: u32) {
    let num: u32 = rand::thread_rng().gen_range(min..max);
    print!("{}) Quel est le numéro {} en français? ",q, num);
    io::stdout().flush().unwrap();
    let mut number = String::new();
    let mut rand_french = String::new();
    conversion(&mut rand_french, num);
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");
    let re = Regex::new(r"[\s]+").unwrap();
    let modified_num=re.replace_all(number.trim(),"-");
    if modified_num.eq(&rand_french) {
        println!("C'est correct!");
    } else {
        println!("La bonne réponse est '{}'.", rand_french);
    }
}

fn main() {
    println!("Premier niveau L1");
    for n in 1..21 {
        question(n, 1 , 11);
    }
    println!("Niveau deux L2");
    for n in 21..36 {
        question(n, 11, 21);
    }
    for n in 36..41 {
        question(n, 1, 21);
    }
    println!("Niveau trois L3");
    for n in 41..56 {
        question(n, 21, 30);
    }
    for n in 56..61 {
        question(n, 1, 30);
    }
    println!("Niveau quatre L4");
    for n in 61..76 {
        question(n, 30, 40);
    }
    for n in 76..81 {
        question(n, 1, 40);
    }
    println!("Niveau cinq L5");
    for n in 81..96 {
        question(n, 40, 50);
    }
    for n in 96..101 {
        question(n, 1, 50);
    }
    println!("Niveau six L6");
    for n in 101..116 {
        question(n, 50, 60);
    }
    for n in 116..121 {
        question(n, 1, 60);
    }
    println!("Niveau sept L7");
    for n in 121..136 {
        question(n, 60, 80);
    }
    for n in 136..141 {
        question(n, 1, 80);
    }
    println!("Niveau huit L8");
    for n in 141..156 {
        question(n, 80, 100);
    }
    for n in 156..161 {
        question(n, 1, 100);
    }
    println!("Niveau neuf L9");
    for n in 161..176 {
        question(n, 100, 200);
    }
    for n in 176..181 {
        question(n, 1, 200);
    }
    println!("Niveau dix L10");
    for n in 181..196 {
        question(n, 200, 1000);
    }
    for n in 196..201 {
        question(n, 1, 1000);
    }
    println!("Niveau onze L11");
    for n in 201..216 {
        question(n, 1000, 2000);
    }
    for n in 216..221 {
        question(n, 1, 2000);
    }
    println!("Niveau douze L12");
    for n in 221..236 {
        question(n, 2001, 10000);
    }
    for n in 236..241 {
        question(n, 1, 10000);
    }
}
