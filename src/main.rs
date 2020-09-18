fn main() {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let search_text = File::open("./data/word_search.txt").unwrap();
    let reader = BufReader::new(&search_text);

    let mut board: Vec<Vec<char>> = vec![];
    for (i, line) in reader.lines().enumerate() {
        board.push(vec![]);

        for character in line.unwrap().chars() {
            board[i].push(character);
        }
    }

    let titles = File::open("./data/titles_pl.txt").unwrap();
    let reader = BufReader::new(&titles);

    let mut end_report = String::default();

    for line in reader.lines() {
        //Debug: intentionally first element

        let line_lower = line.unwrap().to_lowercase();
        let mut found_words = String::default();

        for word in line_lower.split(" ") {
            if word.len() < 3 {
                continue;
            }

            if find(&board, &String::from(word)) {
                found_words.push_str(word);
                found_words.push_str(" ");
            }
        }

        if found_words.len() > 0 {
            let communicate = format!("Word: {} ({})\r\n", line_lower, found_words);

            print!("{}", communicate);
            if !end_report.contains(&communicate) {
                end_report.push_str(&communicate);
            }
        }
    }

    println!("");
    println!("\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\");
    println!("\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\");
    println!("END REPORRT: \n\r{}", end_report);
}

fn find(board: &Vec<Vec<char>>, word: &String) -> bool {
    find_horizontal(board, word) || find_vertical(board, word) || find_diagonal1(board, word) || find_diagonal2(board, word)
}

fn find_horizontal(board: &Vec<Vec<char>>, word: &String) -> bool {
    let mut found = false;

    for (i, line) in board.iter().enumerate() {
        let line_string: String = line.into_iter().collect();

        if line_string.contains(word) {
            println!("{} in line {} horizontal.", word, i);
            found = true;
        }
        let line_string: String = line.into_iter().rev().collect();
        if line_string.contains(word) {
            println!("{} in line {} horizontal backwards.", word, i);
            found = true;
        }
    }

    found
}

fn find_vertical(board: &Vec<Vec<char>>, word: &String) -> bool {
    let mut found = false;

    let height = board.len();
    let width = board[0].len();

    for i in 0..width {
        let mut line = String::default();

        for j in 0..height {
            line.push(board[j][i])
        }

        if line.contains(word) {
            println!("{} in line {} vertical.", word, i);
            found = true;
        }
        let line_rev: String = line.chars().rev().collect();
        if line_rev.contains(word) {
            println!("{} in line {} vertical backwards.", word, i);
            found = true;
        }
    }

    found
}

fn find_diagonal1(board: &Vec<Vec<char>>, word: &String) -> bool {
    let mut found = false;

    let height = board.len();
    let width = board[0].len();

    for i in 0..(height + width - 1) {
        let mut line = String::default();

        let mut height_counter: i32 = std::cmp::min(i as i32, height as i32 - 1);
        let mut width_counter = i - height_counter as usize;

        while width_counter < width && height_counter >= 0 {
            line.push(board[height_counter as usize][width_counter]);

            width_counter += 1;
            height_counter -= 1;
        }

        if line.contains(word) {
            println!("{} in line {} diagonal1.", word, i);
            found = true;
        }
        let line_rev: String = line.chars().rev().collect();
        if line_rev.contains(word) {
            println!("{} in line {} diagonal1 backwards.", word, i);
            found = true;
        }
    }

    found
}

fn find_diagonal2(board: &Vec<Vec<char>>, word: &String) -> bool {
    let mut found = false;

    let height = board.len();
    let width = board[0].len();

    for i in 0..(height + width - 1) {
        let mut line = String::default();

        let mut height_counter = if i >= width {i - width + 1} else {0};
        let mut width_counter = if i < width {i} else {0};

        while width_counter < width && height_counter < height {
            line.push(board[height_counter as usize][width_counter]);

            width_counter += 1;
            height_counter += 1;
        }

        if line.contains(word) {
            println!("{} in line {} diagonal2.", word, i);
            found = true;
        }
        
        let line_rev: String = line.chars().rev().collect();
        if line_rev.contains(word) {
            println!("{} in line {} diagonal2 backwards.", word, i);
            found = true;
        }
    }

    found
}
