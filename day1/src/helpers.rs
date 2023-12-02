// returns (number, index)
pub fn find_spelled_number(line: &str, find_last: bool) -> Option<(&str, usize)> {
    let spelled_nums : [(&str, &str);9] = [
        ("one", "1"), 
        ("two", "2"), 
        ("three", "3"), 
        ("four", "4"), 
        ("five", "5"), 
        ("six", "6"), 
        ("seven", "7"), 
        ("eight", "8"), 
        ("nine", "9")
    ];

    let mut result : Option<(&str, usize)> = None;


    for spelled_num in spelled_nums {
        let mut found: Option<usize> = None; // compiler returns warning because it gets replaced
                                             // before being read... 
        if find_last {
            found = line.rfind(spelled_num.0);
        } else {
            found = line.find(spelled_num.0);
        }

        if found.is_none() {
            continue;
        }

        if result.is_none() || ( 
            find_last && found.unwrap() > result.unwrap().1
        ) || (
            !find_last && found.unwrap() < result.unwrap().1
        ) {
            result = Some((
                spelled_num.1,
                found.unwrap()
            ));
        }
    }
    return result;
}
