use std::collections::HashMap;
// use unicode_segmentation::UnicodeSegmentation;
fn main() {
    
    println!("Hello, world!");

    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2); 

    assert_eq!(vec.len(), 2);
    assert_eq!(vec[0], 1);

    assert_eq!(vec.pop(), Some(2));
    assert_eq!(vec.len(), 1);

    vec[0] = 7;
    assert_eq!(vec[0], 7);

    vec.extend([1,2,3]);

    for x in &vec {
        println!("{x}")
    }

    let mut vec1 = vec![1, 2, 3];
    vec1.push(4);
    let vec2 = Vec::from([1, 2, 3, 4]);
    assert_eq!(vec1, vec2);
    // vec2.pop();
    // assert_eq!(vec1, vec2);

    let vec = vec![0; 5];
    assert_eq!(vec, [0, 0, 0, 0, 0]);

    // The following is equivalent, but potentially slower:
    let mut vec = Vec::with_capacity(5);
    vec.resize(5, 0);
    assert_eq!(vec, [0, 0, 0, 0, 0]);

    // Use a Vec<T> as an efficient stack:

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        // Prints 3, 2, 1
        println!("{top}");

    }
    
    let mut v: Vec<u32> = vec![4, 6, 2, 3, 1, 8, 9, 7, 5, 8];

    let v1: Vec<u32> = vec![5, 6, 5, 3, 4, 2, 6, 7, 9, 3, 4, 2, 1, 4, ];

    v.extend(&v1);

    fn median_mode(x: &mut [u32]) -> (u32, u32) {
        let mut map = HashMap::new();
        let mut mode = 0;
        let mut freq = 0;
        x.sort_unstable();
        println!("This is the sorted vector {:?}", x);
        let len = x.len();
        let median: u32 = if len % 2 == 0 {
                                let mid1 = x[(len / 2) - 1];
                                let mid2 = x[len / 2];
                                (mid1 + mid2) / 2
                            } else {
                                x[len / 2]
                            };


        for n in x {
            let count = map.entry(*n).or_insert(0);
            *count += 1; 
        }

        for (key, val) in map.iter() {
           println!("key: {}, value: {}", key, val);

           if freq < *val {
            freq = *val;
            mode = *key;
           }
        }

        let result: (u32, u32) = (median, mode);
        result

    }


    println!("The median and mode for the given list are {:?}", median_mode(&mut v));

    let b = "The cat the fat cat the fat cat sat on a mouse";
    println!("The string in pig latin is {}", pig_latin(String::from(b)));

}


// Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay”
// is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead 
// (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!

fn pig_latin(s: String) -> String  {
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