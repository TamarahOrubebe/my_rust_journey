fn main() {

    let b = "A cat a fat cat a fat cat sat on a mouse";
    println!("the pig-latin of {} is {}", b, pig_latin(b) );
}

// Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” 
// is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end 
// (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!

fn pig_latin(s: &str) -> String  {
    let v: Vec<&str> = s.split(' ').collect();
    let mut v1: Vec<String> = Vec::new();

    fn is_vowel(c: char) -> bool {
        match c.to_ascii_lowercase() {
            'a' | 'e' | 'i' | 'o' | 'u' => true,
            _ => false
        }
    }
    for  word in v {
       println!("This is what each word looks like {}", word);

        for char in word.chars() {

            if is_vowel(char) {
                let mut new_word: String = String::from(word); 
                new_word.push_str("-hay");
                v1.push(new_word);
                break;
            } else {
                let mut new_word: String = String::from(word); 
                // new_word.push(char)
                new_word.push_str( &format!("-{}ay", char));
                new_word.remove(0);
                v1.push(new_word);
                break;
            }
            
        };
        
    }

     return v1.join(" ");
    //  println!("the string is {:?}", v1);

}

