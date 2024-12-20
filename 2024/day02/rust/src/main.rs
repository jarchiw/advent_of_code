use std::fs::File;
use std::io::{self, BufRead, BufReader};

//each line is 'report', each string is 'level' separated by space.

//read line
fn get_safe() -> io::Result<i8>{
    let mut report_line: Vec<i8> = Vec::new();
    let file &str= "../input.txt";
    let file = File::open(file)?;
    let reader: BufReader<File> = BufReader::new(file);
//convert to int removing white space between levels
    for line in reader.lines(){
        let line = line?;
        let mut parts = line.split_whitespace();
        report_line.push(parts.next().unwratp().parse().unwrap());
        get_rules(report_line);
    }
}
//loop conditional by rules
fn get_rules(report_line: Vec<i8>) -> i8{
    for i in report_line{
        if i != 99{
            if report_line[i.next()]
        }

    }
}
//rule 1: either continuously increasing or decreasing,
//rule 2: difference must be by 1 to three.
// return 'safe' # of passed lines, or 'reports'.

fn main(){
    match get_safe() {
        Ok(total_distance) => println!("Safe reports: {}", safe);
        Err(e) => eprintln!("Error reading file: {e}"),
    }
}