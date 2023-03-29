pub fn run() {
    let initial_str = String::from("At vero eos et accusamus et iusto odio dignissimos ducimus qui blanditiis praesentium voluptatum deleniti atque corrupti quos dolores et quas molestias excepturi sint occaecati cupiditate non provident, similique sunt in culpa qui officia deserunt mollitia animi, id est laborum et dolorum fuga.");
    let vowels = vec!["a", "e", "i", "o", "u"];
    for s in initial_str.split_ascii_whitespace() {
        // if s.starts_with(pat)
        let first_letter: Vec<char> = s.chars().collect();
        let first_letter: char = first_letter[0];
        println!("-----------------");
        println!("{s}");
        let other_values = match s.get(1..) {
            Some(i) => i.to_string(),
            None => String::from(""),
        };

        let mut final_string = String::from("");

        if vowels
            .iter()
            .any(|e| e.to_string() == first_letter.to_string())
        {
            final_string.push_str(&format!("{s}-hay"));
        } else {
            final_string.push_str(&format!("{other_values}-{first_letter}ay"));
        }

        println!("{final_string}");
    }
} //end function run
