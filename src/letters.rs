use std::collections::HashMap;

#[rustfmt::skip]
//  ██  ███   ██       ████ ████  ██  █  █        ██ █  █ █             ██  ███      ███   ███      █  █                █   █████
// █  █ █  █ █  █      █    █    █  █ █  █         █ █ █  █            █  █ █  █     █  █ █         █  █                █   █   █
// █  █ ███  █         ███  ███  █    ████         █ ██   █            █  █ █  █     █  █ █         █  █                 █ █   █ 
// ████ █  █ █         █    █    █ ██ █  █         █ █ █  █            █  █ ███      ███   ██       █  █                  █   █  
// █  █ █  █ █  █      █    █    █  █ █  █      █  █ █ █  █            █  █ █        █ █     █      █  █                  █  █   
// █  █ ███   ██       ████ █     ███ █  █       ██  █  █ ████          ██  █        █  █ ███        ██                   █  ████

#[rustfmt::skip]
lazy_static! {
    static ref LETTERS: HashMap<Vec<bool>, char> = {
        let mut letters = HashMap::new();
        letters.insert(vec![
            false, true, true, false, false,
            true, false, false, true, false,
            true, false, false, true, false,
            true, true, true, true, false,
            true, false, false, true, false,
            true, false, false, true, false],
            'A');
        
        letters.insert(vec![
            true, true, true, false, false,
            true, false, false, true, false,
            true, true, true, false, false,
            true, false, false, true, false,
            true, false, false, true, false,
            true, true, true, false, false],
            'B');
        
        letters.insert(vec![
            false, true, true, false, false,
            true, false, false, true, false,
            true, false, false, false, false,
            true, false, false, false, false,
            true, false, false, true, false,
            false, true, true, false, false],
            'C');
        
        letters.insert(vec![
            false, false, false, false, false,
            false, false, false, false, false,
            false, false, false, false, false,
            false, false, false, false, false,
            false, false, false, false, false,
            false, false, false, false, false],
            'D');
        
        letters.insert(vec![
            true, true, true, true, false,
            true, false, false, false, false,
            true, true, true, false, false,
            true, false, false, false, false,
            true, false, false, false, false,
            true, true, true, true, false],
            'E');
        
        letters.insert(vec![
            true, true, true, true, false,
            true, false, false, false, false,
            true, true, true, false, false,
            true, false, false, false, false,
            true, false, false, false, false,
            true, false, false, false, false],
            'F');
        
        letters.insert(vec![
            false, true, true, false, false,
            true, false, false, true, false,
            true, false, false, false, false,
            true, false, true, true, false,
            true, false, false, true, false,
            false, true, true, true, false],
            'G');
        
        letters.insert(vec![
            true, false, false, true, false,
            true, false, false, true, false,
            true, true, true, true, false,
            true, false, false, true, false,
            true, false, false, true, false,
            true, false, false, true, false],
            'H');
        
        letters.insert(vec![
            false, false, false, false, false,
            false, false, false, false, false,
            false, false, false, false, false,
            false, false, false, false, false,
            false, false, false, false, false,
            false, false, false, false, false],
            'I');
        
        letters.insert(vec![
            false, false, true, true, false,
            false, false, false, true, false,
            false, false, false, true, false,
            false, false, false, true, false,
            true, false, false, true, false,
            false, true, true, false, false],
            'J');
        
        letters.insert(vec![
            true, false, false, true, false,
            true, false, true, false, false,
            true, true, false, false, false,
            true, false, true, false, false,
            true, false, true, false, false,
            true, false, false, true, false],
            'K');
        
        letters.insert(vec![
            true, false, false, false, false,
            true, false, false, false, false,
            true, false, false, false, false,
            true, false, false, false, false,
            true, false, false, false, false,
            true, true, true, true, false],
            'L');
        
        letters.insert(vec![
            false, false, false, false, false,
            false, false, false, false, false,
            false, false, false, false, false,
            false, false, false, false, false,
            false, false, false, false, false,
            false, false, false, false, false],
            'M');
        
        letters.insert(vec![
            false, false, false, false, false,
            false, false, false, false, false,
            false, false, false, false, false,
            false, false, false, false, false,
            false, false, false, false, false,
            false, false, false, false, false],
            'N');
        
        letters.insert(vec![
            false, true, true, false, false,
            true, false, false, true, false,
            true, false, false, true, false,
            true, false, false, true, false,
            true, false, false, true, false,
            false, true, true, false, false],
            'O');
        
        letters.insert(vec![
            true, true, true, false, false,
            true, false, false, true, false,
            true, false, false, true, false,
            true, true, true, false, false,
            true, false, false, false, false,
            true, false, false, false, false],
            'P');
        
        letters.insert(vec![
            false, false, false, false, false,
            false, false, false, false, false,
            false, false, false, false, false,
            false, false, false, false, false,
            false, false, false, false, false,
            false, false, false, false, false],
            'Q');
        
        letters.insert(vec![
            true, true, true, false, false,
            true, false, false, true, false,
            true, false, false, true, false,
            true, true, true, false, false,
            true, false, true, false, false,
            true, false, false, true, false],
            'R');
        
        letters.insert(vec![
            false, true, true, true, false,
            true, false, false, false, false,
            true, false, false, false, false,
            false, true, true, false, false,
            false, false, false, true, false,
            true, true, true, false, false],
            'S');
        
        letters.insert(vec![
            false, false, false, false, false,
            false, false, false, false, false,
            false, false, false, false, false,
            false, false, false, false, false,
            false, false, false, false, false,
            false, false, false, false, false],
            'T');
        
        letters.insert(vec![
            true, false, false, true, false,
            true, false, false, true, false,
            true, false, false, true, false,
            true, false, false, true, false,
            true, false, false, true, false,
            false, true, true, false, false],
            'U');
        
        letters.insert(vec![
            false, false, false, false, false,
            false, false, false, false, false,
            false, false, false, false, false,
            false, false, false, false, false,
            false, false, false, false, false,
            false, false, false, false, false],
            'V');
        
        letters.insert(vec![
            false, false, false, false, false,
            false, false, false, false, false,
            false, false, false, false, false,
            false, false, false, false, false,
            false, false, false, false, false,
            false, false, false, false, false],
            'W');
        
        letters.insert(vec![
            false, false, false, false, false,
            false, false, false, false, false,
            false, false, false, false, false,
            false, false, false, false, false,
            false, false, false, false, false,
            false, false, false, false, false],
            'X');
        
        letters.insert(vec![
            true, false, false, false, true,
            true, false, false, false, true,
            false, true, false, true, false,
            false, false, true, false, false,
            false, false, true, false, false,
            false, false, true, false, false],
            'Y');
        
        letters.insert(vec![
            true, true, true, true, false,
            false, false, false, true, false,
            false, false, true, false, false,
            false, true, false, false, false,
            true, false, false, false, false,
            true, true, true, true, false],
            'Z');
        letters
    };
}

pub fn recognize_letters(image: &[bool]) -> String {
    let mut image = image.to_vec();
    let mut column_count = image.len() / 6;
    for _ in (column_count % 5)..5 {
        for x in 0..6 {
            let x = 6 - x;
            image.insert(x * column_count, false);
        }
        column_count = image.len() / 6;
    }
    let char_count = column_count / 5;

    let mut offset = 0;
    'outer: loop {
        for row in 0..6 {
            if image[row * column_count + offset] {
                break 'outer;
            }
        }
        offset += 1;
    }

    // collect the characters…
    let mut rv = vec![];
    for i in 0..char_count {
        let mut letter = vec![];
        for row in 0..6 {
            for column in 0..5 {
                // print!("{}", if image[offset + row * column_count + column + 5 * i] { '█' } else { ' ' });
                letter.push(image[offset + row * column_count + column + 5 * i]);
            }
            // println!();
        }
        // println!("  => {:?}", LETTERS[&letter]);
        rv.push(LETTERS[&letter]);
    }
    rv.into_iter().collect()
}
