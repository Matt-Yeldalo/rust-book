mod slices {
    pub fn run() {
        println!("Enter word");
        let mut word = String::new();
        std::io::stdin()
            .read_line(&mut word)
            .expect("Failed to read line");
        println!("First word: {}", first_word(&word))
    }

    fn first_word(s: &str) -> &str {
        let index = first_space(&s);
        &s[0..index]
    }

    // Using split_at
    // fn first_word(s: &String) -> String {
    //     let index = first_space(&s);
    //     let (a, _b) = s.split_at(index);
    //     a.to_string()
    // }

    fn first_space(s: &str) -> usize {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return i;
            }
        }
        return s.len();
    }
}
