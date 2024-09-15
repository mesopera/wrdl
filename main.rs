use std::fs::{self, File};
use std::io::{self, prelude::*, BufReader, BufWriter, Seek, SeekFrom};
use std::iter::Enumerate;
use std::path::Path;
use rand::Rng;
use colored::*;

enum Choice {
    ExactMatch,
    PartialMatch,
    NoMatch,
}

fn main() {
    println!("{}", "<----wrdl---->".bold().green());
    let parsed_file = "words_parsed.txt";
    let keyWord = random_select(&parsed_file).unwrap();             //randomly selects keyWord from parsed file
    //println!("{keyWord:?}");                                      //remove comment for debugging
    for iter in 0..5 {
        let mut finish_Counter = 0;

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");                         //input

        let word_vector: Vec<char> = guess.chars().collect();       //slice input

        for i in 0..4 {
            //compares and stores letters with matching indices
            let matching_Vector: Vec<_> = keyWord.match_indices(word_vector[i]).collect();

            let mut j = 0;
            let mut choice = Choice::NoMatch;
            let mut vector_Length = matching_Vector.len();

            while vector_Length != 0 {
                if i == matching_Vector[j].0 {                      //if indices match
                    choice = Choice::ExactMatch;
                    finish_Counter += 1;                            //keeps track of number of exact matches
                    break;
                }

                else if word_vector[i].to_string() == matching_Vector[j].1 {
                    choice = Choice::PartialMatch;                  //if the letter exists at a different indice
                }
                j = j + 1;
                vector_Length = vector_Length - 1;
            }
            match choice {
                Choice::ExactMatch => print!("{} ", word_vector[i].to_string().green().bold()),
                Choice::PartialMatch => print!("{} ", word_vector[i].to_string().yellow().bold()),
                Choice::NoMatch => print!("{} ", word_vector[i].to_string().bold()),
            }
        }
        println!();
        if finish_Counter == 4 {                                    //winning sequence
            println!("{}","<----You win!---->".yellow());
            println!("{} = {}", "Number of tries", format!("{}", (iter + 1)).red().bold());
            return;
        }
    }
    println!("{}", "BOOOO!!! XD\nYou Fail!".red().bold());
    println!("{}, {}", "The answer was", keyWord.green());
    return;
}

fn random_select(input_path: &str) -> io::Result<String> {
    let number_of_bytes_per_line = 6;
    let range_max = fs::metadata(input_path).unwrap().len() / number_of_bytes_per_line;
    let seek_pos = rand::thread_rng().gen_range(1..=range_max) * number_of_bytes_per_line;

    let mut words = File::open(input_path)?;
    let _ = words.seek(SeekFrom::Start(seek_pos));
    let mut buffer = [0u8; 4];
    let _ = words.read_exact(&mut buffer);
    let word = unsafe {
        String::from_utf8_unchecked(buffer.to_vec())
    };
    Ok(word)
}

//////////only required during initial parse
/*
fn parse(input_path: &str, output_path: &str) -> io::Result<()> {
    let words = File::open(input_path)?;
    let parsed_words = File::create(output_path)?;

    let mut reader = BufReader::new(words);
    let mut writer = BufWriter::new(parsed_words);

    let mut line = String::new();

    loop {
        line.clear();

        match reader.read_line(&mut line)? {
            0 => break,
            6 => {
                writer.write_all(line.as_bytes())?;
            },
            _ => (),
        }
    }
    Ok(())
}
*/
