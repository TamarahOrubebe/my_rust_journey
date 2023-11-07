use std::collections::HashMap;
use std::io;

fn main () {
    let mut v: Vec<u32> = vec![4, 6, 2, 3, 1, 8, 9, 7, 5, 8];

    let v1: Vec<u32> = vec![5, 6, 5, 3, 4, 2, 6, 7, 9, 3, 4, 2, 1, 4, ];

    v.extend(&v1);

    let mut company: HashMap<String, Vec<String>> = HashMap::new();
    

    loop {
        println!("Enter a command e.g Add Sally to Engineering or Add Amir to Sales, or List all company employees, or List
        employees in department_name");
        
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input).expect("failed to read line");

        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let parts: Vec<&str> = input.split(" ").collect();

        if parts.len() < 4 {
            println!("Pls enter a valid commad e.g Add Name to Department, or List employees in Department
            , or List all company employees");
            continue;
        }

        let name = parts[1];
        let department = parts[3];

        match parts[0] {
            "Add" => company.entry(department.to_string()).or_insert(Vec::new()).push(name.to_string()),
            "List" => {
                if department == "employees" {
                    list_all_employees(&company);
                } else {
                    list_employees_per_department(&company, department);
                }
            },
            _ => println!("Please enter a valid comman using Add or List")

        }



    }


    fn list_all_employees(company: &HashMap<String, Vec<String>>) {
        let mut employee_list: Vec<String> = Vec::new();

        for department_employees in company.values() {
            employee_list.extend(department_employees.iter().cloned());
        }
        employee_list.sort();

        println!("The names of all the company employees are {:?}", employee_list);
    }

    fn list_employees_per_department(company: &HashMap<String, Vec<String>>, department: &str ) {
        
        if let Some(employees) = company.get(department) {

            let mut department_employees = employees.clone();

            department_employees.sort();
            println!("The names of employees in the {} department are: {:?}", department, department_employees);
        } else {
            println!("Department {} not found", department);
        }

    }

    
}

// Given a list of integers, use a vector and return the median (when sorted, the value in the middle position)
// and mode (the value that occurs most often; a hash map will be helpful here) of the list.

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



    // Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” 
    // is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end 
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


// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department
// in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.”
// Then let the user retrieve a list of all people in a department or all people in the company by department, 
// sorted alphabetically.
