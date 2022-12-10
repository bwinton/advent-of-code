use once_cell::sync::Lazy;
use std::collections::HashMap;

#[rustfmt::skip]
//  ██  ███   ██       ████ ████  ██  █  █        ██ █  █ █             ██  ███      ███   ███      █  █                █   █████
// █  █ █  █ █  █      █    █    █  █ █  █         █ █ █  █            █  █ █  █     █  █ █         █  █                █   █   █
// █  █ ███  █         ███  ███  █    ████         █ ██   █            █  █ █  █     █  █ █         █  █                 █ █   █ 
// ████ █  █ █         █    █    █ ██ █  █         █ █ █  █            █  █ ███      ███   ██       █  █                  █   █  
// █  █ █  █ █  █      █    █    █  █ █  █      █  █ █ █  █            █  █ █        █ █     █      █  █                  █  █   
// █  █ ███   ██       ████ █     ███ █  █       ██  █  █ ████          ██  █        █  █ ███        ██                   █  ████

#[rustfmt::skip]
static LETTERS: Lazy<HashMap<Vec<bool>, char>> = Lazy::new(|| {
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
    
    // letters.insert(vec![
    //     false, false, false, false, false,
    //     false, false, false, false, false,
    //     false, false, false, false, false,
    //     false, false, false, false, false,
    //     false, false, false, false, false,
    //     false, false, false, false, false],
    //     'X');
    
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
});

pub fn recognize_letters(image: &[bool]) -> String {
    let image = image.to_vec();
    let column_count = image.len() / 6;
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

    let mut picture = String::new();
    for (index, &value) in image.iter().enumerate() {
        picture.push(if value { '█' } else { ' ' });
        if (index + 1) % column_count == 0 {
            picture.push('.');
            picture.push('\n');
        }
    }

    if (column_count - offset) % 5 != 0 {
        // Remove all the blank rows at the end, then add one, and see how that affects it.
        // dbg!((column_count - offset) % 5);
        // println!("picture:\n{}", picture);
    }

    // collect the characters…
    let mut rv = vec![];
    for i in 0..char_count {
        let mut letter = vec![];
        for row in 0..6 {
            for column in 0..5 {
                letter.push(image[offset + row * column_count + column + 5 * i]);
            }
        }
        if LETTERS.get(&letter).is_none() {
            // We didn't find one of the letters, so return the picture.
            return picture;
        }
        rv.push(LETTERS[&letter]);
    }
    rv.into_iter().collect()
}
