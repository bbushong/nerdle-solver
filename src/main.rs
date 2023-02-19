use std::cmp::Ordering;
use std::io;
use std::ops::Add;

fn plus_one(x: i32) -> i32 {
    x + 1;
    return 50;
}

fn all_possibles(mut possibles: [[bool; 15]; 8], character_set: [char; 15]) -> String{
    let mut can_be = String::new();
    for line in possibles {
        let mut x = 0;
        for character in line {
            if character == true {
                can_be.push(character_set[x])
            }
            x += 1;
        }
    }
    let mut can_be: Vec<char> = can_be.chars().collect();

    // gi and gc refer to the guesses given to the user to enter...
    can_be.sort();
    can_be.dedup();
    let return_string: String = can_be.iter().collect::<String>();
    return return_string;
}

fn two_group_answer_check(mut group1: String, operand1: char, group2: String, mut possibles: [[bool; 15]; 8], character_set: [char; 15]) -> bool {
    let mut text_answer = format!("{group1}{operand1}{group2}=");
    let answer;
    let g1: f32 = match group1.trim().parse() {
        Ok(num) => num,
        Err(_) => return false,
    };
    let g2: f32 = match group2.trim().parse() {
        Ok(num) => num,
        Err(_) => return false,
    };

    match operand1 {
        '+' => answer = g1 + g2,
        '-' => answer = g1 - g2,
        '*' => answer = g1 * g2,
        '/' => {
            if g1 > 0.0 && g2 > 0.0 {
                answer = g1 / g2
            }
            else {
                return false
            }
        },
        _ => return false,
    }
    text_answer = format!("{text_answer}{answer}");
    if text_answer.len() != 8 {
        //println!("{text_answer} - not the correct # characters!");
        return false
    }
    else {
        //print!("{text_answer} - *** correct # characters!");
        //return true;
    }

    // okay, now check if EVERYTHING matches...
    for (i, c) in text_answer.chars().enumerate() {
        let mut pass = false;
        for (si, s) in character_set.into_iter().enumerate() {
            if s == c {
                pass = true;
                match i {
                    0 => {
                        if possibles[0][si] != true {
                            return false;
                        }
                    },
                    1 => {
                        if possibles[1][si] != true {
                            return false;
                        }
                    },
                    2 => {
                        if possibles[2][si] != true {
                            return false;
                        }
                    },
                    3 => {
                        if possibles[3][si] != true {
                            return false;
                        }
                    },
                    4 => {
                        if possibles[4][si] != true {
                            return false;
                        }
                    },
                    5 => {
                        if possibles[5][si] != true {
                            return false;
                        }
                    },
                    6 => {
                        if possibles[6][si] != true {
                            return false;
                        }
                    },
                    7 => {
                        if possibles[7][si] != true {
                            return false;
                        }
                    },
                    _ => return false
                }
            }
        }
        if pass == false {
            return false;
        }
    }
    // if we are here, we made it all the way through checking!
    //println!("{text_answer} - *** things are in the right place!");

    // okay, we need to figure out how many unique characters are in this... and does this answer fill the need?
    let mut text_answer_vec: Vec<char> = text_answer.chars().collect();

    // gi and gc refer to the guesses given to the user to enter...
    text_answer_vec.sort();
    text_answer_vec.dedup();

    if all_possibles(possibles, character_set).len() == text_answer_vec.len() {
        println!("{text_answer} - adds up, has the right number of chars, and has the right characters!");
    }



    return true;
}

fn three_group_answer_check(mut group1: String, operand1: char, group2: String,  operand2: char, group3: String, mut possibles: [[bool; 15]; 8], character_set: [char; 15]) -> bool {
    let mut text_answer = format!("{group1}{operand1}{group2}{operand2}{group3}=");
    let answer;
    let g1: f32 = match group1.trim().parse() {
        Ok(num) => num,
        Err(_) => return false,
    };
    let g2: f32 = match group2.trim().parse() {
        Ok(num) => num,
        Err(_) => return false,
    };
    let g3: f32 = match group3.trim().parse() {
        Ok(num) => num,
        Err(_) => return false,
    };

    //println!("{} | {} | {} | {} | {}", g1, operand1, g2, operand2, g3);

    match operand1 {
        '+' => {
            match operand2 {
                '+' => answer = g1 + g2 + g3,
                '-' => answer = g1 + g2 - g3,
                '*' => answer = g1 + g2 * g3,
                '/' => {
                    if g3 > 0.0 {
                        answer = g1 + g2 / g3;
                    } else {
                        return false;
                    }
                },
                _ => return false,
            }
        },
        '-' => {
            match operand2 {
                '+' => answer = g1 - g2 + g3,
                '-' => answer = g1 - g2 - g3,
                '*' => answer = g1 - g2 * g3,
                '/' => {
                    if g3 > 0.0 {
                        answer = g1 - g2 / g3;
                    } else {
                        return false;
                    }
                },
                _ => return false,
            }
        },
        '*' => {
            match operand2 {
                '+' => answer = g1 * g2 + g3,
                '-' => answer = g1 * g2 - g3,
                '*' => answer = g1 * g2 * g3,
                '/' => {
                    if g3 > 0.0 {
                        answer = g1 * g2 / g3;
                    } else {
                        return false;
                    }
                },
                _ => return false,
            }
        },
        '/' => {
            if g2 > 0.0 {
                match operand2 {
                    '+' => answer = g1 / g2 + g3,
                    '-' => answer = g1 / g2 - g3,
                    '*' => answer = g1 / g2 * g3,
                    '/' => {
                        if g3 > 0.0 {
                            answer = g1 / g2 / g3;
                        } else {
                            return false;
                        }
                    },
                    _ => return false,
                }
            }
            else {
                return false
            }
        },
        _ => return false,
    }

    text_answer = format!("{text_answer}{answer}");
    //println!("{text_answer} - found this...");
    if text_answer.len() != 8 {
        //println!("{text_answer} - not the correct # characters!");
        return false
    }

    // okay, now check if EVERYTHING matches...
    for (i, c) in text_answer.chars().enumerate() {
        let mut pass = false;
        for (si, s) in character_set.into_iter().enumerate() {
            if s == c {
                pass = true;
                match i {
                    0 => {
                        if possibles[0][si] != true {
                            return false;
                        }
                    },
                    1 => {
                        if possibles[1][si] != true {
                            return false;
                        }
                    },
                    2 => {
                        if possibles[2][si] != true {
                            return false;
                        }
                    },
                    3 => {
                        if possibles[3][si] != true {
                            return false;
                        }
                    },
                    4 => {
                        if possibles[4][si] != true {
                            return false;
                        }
                    },
                    5 => {
                        if possibles[5][si] != true {
                            return false;
                        }
                    },
                    6 => {
                        if possibles[6][si] != true {
                            return false;
                        }
                    },
                    7 => {
                        if possibles[7][si] != true {
                            return false;
                        }
                    },
                    _ => return false
                }
            }
        }
        if pass == false {
            return false;
        }
    }
    // if we are here, we made it all the way through checking!
    //println!("{text_answer} - *** things are in the right place!");

    // okay, we need to figure out how many unique characters are in this... and does this answer fill the need?
    let mut text_answer_vec: Vec<char> = text_answer.chars().collect();

    // gi and gc refer to the guesses given to the user to enter...
    text_answer_vec.sort();
    text_answer_vec.dedup();

    if all_possibles(possibles, character_set).len() == text_answer_vec.len() {
        println!("{text_answer} - adds up, has the right number of chars, and has the right characters!");
    }



    return true;
}

fn one_digit_number(set1: [bool; 15], character_set: [char; 15]) -> String {
    let mut can_be = String::new();
    let mut x = 0;
    for character in set1 {
        if character == true  && x < 10 {
            can_be.push(character_set[x]);
        }
        x += 1;
    }
    return can_be;
}

fn operand(set1: [bool; 15], character_set: [char; 15]) -> String {
    let mut can_be = String::new();
    let mut x = 0;
    for character in set1 {
        if character == true  && x > 9 {
            can_be.push(character_set[x]);
        }
        x += 1;
    }
    return can_be;
}

fn two_digit_number(set1: [bool; 15], set2: [bool; 15], character_set: [char; 15]) -> String {
    let mut can_be = String::new();
    let mut x = 0;
    let chars1 = one_digit_number(set1, character_set);
    let chars2 = one_digit_number(set2, character_set);
    for c1 in chars1.chars() {
        for c2 in chars2.chars() {
            can_be.push(c1);
            can_be.push(c2);
            can_be.push('|');
        }
    }
    can_be.pop();
    return can_be;
}

fn three_digit_number(set1: [bool; 15], set2: [bool; 15], set3: [bool; 15], character_set: [char; 15]) -> String {
    let mut can_be = String::new();
    let mut x = 0;
    let chars1 = one_digit_number(set1, character_set);
    let chars2 = one_digit_number(set2, character_set);
    let chars3 = one_digit_number(set3, character_set);
    for c1 in chars1.chars() {
        for c2 in chars2.chars() {
            for c3 in chars3.chars() {
                can_be.push(c1);
                can_be.push(c2);
                can_be.push(c3);
                can_be.push('|');
            }
        }
    }
    can_be.pop();
    return can_be;
}

fn print_stack(mut possibles: [[bool; 15]; 8], character_set: [char; 15]) {
    let mut i = 1;
    for line in possibles {
        let mut can_be = String::new();
        let mut x = 0;
        for character in line {
            if character == true {
                can_be.push(character_set[x])
            }
            x += 1;
        }
        println!("Box {i} can be: {can_be}");
        i += 1;
    }
}

fn eval_answer(mut possibles: [[bool;15]; 8], character_set: [char; 15], mut question: String, mut answer: String) -> [[bool;15];8] {
    // turn to a vector (whatever that is...)
    let answer: Vec<char> = answer.chars().collect();

    // gi and gc refer to the guesses given to the user to enter...
    for (gi, gc) in question.chars().enumerate() {
        // let's get the character_set position of the character given...
        print!("We are dealing with the character: {gc}");
        let mut character_number: usize = 10;
        let mut i: usize = 0;
        for c in character_set {
            if c == gc {
                character_number = i;
            }
            i += 1;
        }
        print!("... which was found at index {character_number}");
        println!("... and you typed: {}", answer[gi]);

        match answer[gi] {
            'b' => {
                println!("Removing character {gc} which is index {character_number}");
                // means the character isn't anywhere - need to remove it from ALL positions
                possibles[0][character_number] = false;
                possibles[1][character_number] = false;
                possibles[2][character_number] = false;
                possibles[3][character_number] = false;
                possibles[4][character_number] = false;
                possibles[5][character_number] = false;
                possibles[6][character_number] = false;
                possibles[7][character_number] = false;
            }
            'g' => {
                //means the character belongs RIGHT where it is!
                println!("the character {gc}/{character_number} belongs RIGHT where it is {gi}");
                match gi {
                    0 => {
                        possibles[0] = [false; 15];
                        possibles[0][character_number] = true
                    },
                    1 => {
                        possibles[1] = [false; 15];
                        possibles[1][character_number] = true
                    },
                    2 => {
                        possibles[2] = [false; 15];
                        possibles[2][character_number] = true
                    },
                    3 => {
                        possibles[3] = [false; 15];
                        possibles[3][character_number] = true
                    },
                    4 => {
                        possibles[4] = [false; 15];
                        possibles[4][character_number] = true
                    },
                    5 => {
                        possibles[5] = [false; 15];
                        possibles[5][character_number] = true
                    },
                    6 => {
                        possibles[6] = [false; 15];
                        possibles[6][character_number] = true
                    },
                    7 => {
                        possibles[7] = [false; 15];
                        possibles[7][character_number] = true
                    },
                    _ => {
                        println!("Something ain't right!");
                        break; }
                }
            },
            'p' => {
                // means the character CAN'T be HERE.
                println!("the character {gc}/{character_number} can't be where it is {gi}");
                match gi {
                    0 => possibles[0][character_number] = false,
                    1 => possibles[1][character_number] = false,
                    2 => possibles[2][character_number] = false,
                    3 => possibles[3][character_number] = false,
                    4 => possibles[4][character_number] = false,
                    5 => possibles[5][character_number] = false,
                    6 => possibles[6][character_number] = false,
                    7 => possibles[7][character_number] = false,
                    _ => {
                        println!("Something ain't right!");
                        break;
                    }
                }
            }
            _ => { },
        }
    }
    return possibles;
}

fn main() {
    // play with calculations
    let mut guess1 = String::from("9*8-7=65");
    let mut guess2 = String::from("0+12/3=4");
    let mut guess3 = String::new();
    let mut guess4 = String::new();
    let mut guess5 = String::new();
    let mut guess6 = String::new();

    let mut guess1_result = String::new();
    let mut guess2_result = String::new();
    let mut guess3_result = String::new();
    let mut guess4_result = String::new();
    let mut guess5_result = String::new();
    let mut guess6_result = String::new();

    let character_set: [char; 15] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '-', '*', '/', '='];

    let mut possibles = [
        [true; 15],
        [true; 15],
        [true; 15],
        [true; 15],
        [true; 15],
        [true; 15],
        [true; 15],
        [true; 15],
    ];

    print_stack(possibles,character_set);

    println!("Guess1: Please enter the results of: {guess1} using b=black, g=green, p=purple. eg: bgbbppbb");
    io::stdin()
        .read_line(&mut guess1_result)
        .expect("Failed to read line");

    possibles = eval_answer(possibles, character_set, guess1, guess1_result);

    print_stack(possibles,character_set);

    println!("Guess2: Now enter the results of: {guess2} using b=black, g=green, p=purple.");
    io::stdin()
        .read_line(&mut guess2_result)
        .expect("Failed to read line");

    possibles = eval_answer(possibles, character_set, guess2, guess2_result);

    print_stack(possibles,character_set);

    let equal_sign:u32;
    // okay, we know most everything now... Let's find the equal sign first
    if possibles[6][14] == true {
        equal_sign = 7
    }
    else if possibles[5][14] == true {
        equal_sign = 6
    }
    else {
        equal_sign = 5;
    }

    // possible solutions:
    //if "=" is in 7th spot (1 number answer / 6 unknowns)
    if equal_sign == 7 {
        // x-x-xx
        for (p1i, g1) in one_digit_number(possibles[0], character_set).chars().enumerate() {
            for (p2i, g2) in one_digit_number(possibles[2], character_set).chars().enumerate() {
                for (p3i, g3) in two_digit_number(possibles[4], possibles[5], character_set).split('|').enumerate() {
                    for o1 in operand(possibles[1], character_set).chars() {
                        for o2 in operand(possibles[3], character_set).chars() {
                            three_group_answer_check(g1.to_string(), o1, g2.to_string(), o2, g3.to_string(), possibles, character_set);
                        }
                    }
                }
            }
        }
        // xx-x-x
        for (p1i, g1) in two_digit_number(possibles[0], possibles[1], character_set).split('|').enumerate() {
            for (p2i, g2) in one_digit_number(possibles[3], character_set).chars().enumerate() {
                for (p3i, g3) in one_digit_number(possibles[5], character_set).chars().enumerate() {
                    for o1 in operand(possibles[2], character_set).chars() {
                        for o2 in operand(possibles[4], character_set).chars() {
                            //println!("right group: {}|{}|{}|{}|{}", g1.to_string(), o1, g2.to_string(), o2, g3.to_string());
                            three_group_answer_check(g1.to_string(), o1, g2.to_string(), o2, g3.to_string(), possibles, character_set);
                        }
                    }
                }
            }
        }
        // x-xx-x
        for (p1i, g1) in one_digit_number(possibles[0], character_set).chars().enumerate() {
            for (p2i, g2) in two_digit_number(possibles[2], possibles[3], character_set).chars().enumerate() {
                for (p3i, g3) in one_digit_number(possibles[5],character_set).chars().enumerate() {
                    for o1 in operand(possibles[1], character_set).chars() {
                        for o2 in operand(possibles[4], character_set).chars() {
                            three_group_answer_check(g1.to_string(), o1, g2.to_string(), o2, g3.to_string(), possibles, character_set);
                        }
                    }
                }
            }
        }
        // xxx-xx
        for (p1i, g1) in three_digit_number(possibles[0], possibles[1], possibles[2], character_set).split('|').enumerate() {
            for (p2i, g2) in two_digit_number(possibles[4], possibles[5], character_set).split('|').enumerate() {
                for o1 in operand(possibles[3], character_set).chars() {
                    two_group_answer_check(g1.to_string(),o1 , g2.to_string(), possibles, character_set);
                }
            }
        }
        // xx-xxx
        for (p1i, g1) in two_digit_number(possibles[0], possibles[1], character_set).split('|').enumerate() {
            for (p2i, g2) in three_digit_number(possibles[3], possibles[4], possibles[5], character_set).split('|').enumerate() {
                for o1 in operand(possibles[2], character_set).chars() {
                    two_group_answer_check(g1.to_string(),o1 , g2.to_string(), possibles, character_set);
                }
            }
        }
    }
    //if "=" is in the 6th spot (2 number answer / 5 unknowns)
    if equal_sign == 7 {
        // x-x-x
        for (p1i, g1) in one_digit_number(possibles[0], character_set).chars().enumerate() {
            for (p2i, g2) in one_digit_number(possibles[2], character_set).chars().enumerate() {
                for (p3i, g3) in one_digit_number(possibles[4], character_set).chars().enumerate() {
                    for o1 in operand(possibles[1], character_set).chars() {
                        for o2 in operand(possibles[3], character_set).chars() {
                            three_group_answer_check(g1.to_string(), o1, g2.to_string(), o2, g3.to_string(), possibles, character_set);
                        }
                    }
                }
            }
        }
        // xx-xx
        for (p1i, g1) in two_digit_number(possibles[0], possibles[1], character_set).split('|').enumerate() {
            for (p1i, g2) in two_digit_number(possibles[3], possibles[4], character_set).chars().enumerate() {
                for o1 in operand(possibles[2], character_set).chars() {
                    two_group_answer_check(g1.to_string(), o1, g2.to_string(), possibles, character_set);
                }
            }
        }
        // x-xxx
        for (p1i, g1) in one_digit_number(possibles[0], character_set).chars().enumerate() {
            for (p1i, g2) in three_digit_number(possibles[2], possibles[3], possibles[4], character_set).split('|').enumerate() {
                for o1 in operand(possibles[1], character_set).chars() {
                    two_group_answer_check(g1.to_string(), o1, g2.to_string(), possibles, character_set);
                }
            }
        }
        // xxx-x
        for (p1i, g1) in three_digit_number(possibles[0], possibles[1], possibles[2], character_set).split('|').enumerate() {
            for (p1i, g2) in one_digit_number(possibles[4], character_set).chars().enumerate() {
                for o1 in operand(possibles[3], character_set).chars() {
                    two_group_answer_check(g1.to_string(), o1, g2.to_string(), possibles, character_set);
                }
            }
        }
    }
    //if "=" is in the 5th spot (1 number answer / 4 unknowns)
    if equal_sign == 5 {
        // x-xx
        for (p1i, g1) in one_digit_number(possibles[0], character_set).chars().enumerate() {
            for (p1i, g2) in two_digit_number(possibles[2], possibles[3], character_set).split('|').enumerate() {
                for o1 in operand(possibles[2], character_set).chars() {
                    two_group_answer_check(g1.to_string(), o1, g2.to_string(), possibles, character_set);
                }
            }
        }
        // xx-x
        for (p1i, g1) in two_digit_number(possibles[0], possibles[1], character_set).split('|').enumerate() {
            for (p1i, g2) in one_digit_number(possibles[3], character_set).chars().enumerate() {
                for o1 in operand(possibles[2], character_set).chars() {
                    two_group_answer_check(g1.to_string(), o1, g2.to_string(), possibles, character_set);
                }
            }
        }

    }
}