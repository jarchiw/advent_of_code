use std::fs::File;
use std::io::{self, BufRead, BufReader};

//each line is 'report', each string is 'level' separated by space.
//read line
//create Vec split by whitespace
//loop conditional by rules:
//rule 1: either continuously increasing or decreasing,
//rule 2: difference must be by 1 to three.
// return 'safe' # of passed lines.
