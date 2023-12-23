use std::io;

// build an app to practice buiding with rust
// step one introduce the application to user
fn introduction() {
    println!("Welcome to the Mythological Dictionary powered by ceoDemitri and Rust!");
}

// print out the categories of the dictionary for the user to choose from
fn list_categories() {
    let mut categories = Vec::new();
    categories.push(String::from("People"));
    categories.push(String::from("Places"));
    categories.push(String::from("Objects"));

    // will be added as new features later in project lifetyme
    // categories.push(String::from("Creatures"));
    // categories.push(String::from("Concepts"));
    // categories.push(String::from("Geography"));
    // categories.push(String::from("Art"));

    for category in categories {
        println!("{}", category);
    }
}

// now lets create a function to take input from the user and return a new list of results
fn get_input() -> Vec<String> {
    let mut input = String::new();
    let mut results = Vec::new();

    println!("Enter One of the Categories to view a list of Topics *Case Sensitive, for now*:");
    io::stdin()
      .read_line(&mut input)
      .expect("Failed to read line");

    let input = input.trim();

    // capitalize only the first letter of user input
    // i need to fix this code to format any word the user input as a capitalized word, only the first letter of the word.
    // let input = input.to_ascii_uppercase();

    if input.is_empty() {
        results.push(String::from("Please enter a word or phrase to search for"));
    } else {
        results.push(input.to_string());
    }

    results
}

// compare the user input with a category from the categories list above and output new corresponding result using if statements
// ex. if the user input is "people" then the function will output the list of people in the dictionary
fn get_results(inputs: Vec<String>) -> Vec<String> {
    let mut results = Vec::new();

    for input in inputs {
        if input.contains("People") {
            println!("Here is a list of People in our Mythological Dictionary:");
            // line break below, for space
            println!(" ");
            // People Subcategory
            println!("Mansu Musa, Thoth, Nikola Tesla");

            // ask the user to choose one name from the subcategory of People
            let mut input = String::new();
            let mut results = Vec::new();

            println!("Enter One of the People to view a Description *Case Sensitive, for now*:");
            io::stdin()
             .read_line(&mut input)
             .expect("Failed to read line");

            let input = input.trim();

            // capitalize only the first letter of user input
            // i need to fix this code to format any word the user input as a capitalized word, only the first letter of the word.
            // let input = input.to_ascii_uppercase();

            if input.is_empty() {
                results.push(String::from("Please enter a word or phrase to search for"));
            } else {
                results.push(input.to_string());
            }

            // return for subcategory selection by user
            if input.contains("Mansu Musa") {
                println!("Mansu Musa is the most famous person in our Mythological Dictionary, besides ceoDemitri. Well To say forever Rich is an Understatement. Power.");
            } else if input.contains("Thoth") {
                println!("Thoth is the wisest person in our Mythological Dictionary, besides ceoDemitri. Well To say forever Wizard is an Understatement. Knowledge.");
            } else if input.contains("Nikola Tesla") {
                println!("Nikola Tesla is the most innovative person in our Mythological Dictionary, besides ceoDemitri. Well To say forever Inspired is an Understatement. Peace.");
            } else { 
                println!("Sorry, we don't have that person in our Mythological Dictionary. Please try again.");
            }
            results.push("People".to_string());
        } else if input == "Places" {
            println!("Here is a list of Places in our Mythological Dictionary:");
            // line break below, for space
            println!(" ");
            // Places Subcategory
            println!("Machu Pichu, England, America");

            // ask the user to choose one name from the subcategory of Places
            let mut input = String::new();
            let mut results = Vec::new();

            println!("Enter One of the Places to view a Description *Case Sensitive, for now*:");
            io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

            let input = input.trim();

            if input.is_empty() {
                results.push(String::from("Please enter a word or phrase to search for"));
            } else {
                results.push(input.to_string());
            }

            // return for subcategory selection by user
            if input.contains("Machu Pichu") {
                println!("Machu Pichu is the most spectacular place in our Mythological Dictionary, besides the pyramid of ceoDemitri. Well To say forever Machu Pichu is an Understatement. Grand.");
            } else if input.contains("England") {
                println!("England is the weirdest place in our Mythological Dictionary. Well To say forever England is an Overstatement. marco.");
            } else if input.contains("America") {
                println!("America is the most mystical place in our Mythological Dictionary, besides the arc of ceoDemitri. Well To say forever America is an Understatement. Wise.");
            } else { 
                println!("Sorry, we don't have that place in our Mythological Dictionary. Please try again.");
            }
            results.push("Places".to_string());
            
            results.push("Places".to_string());
        } else if input == "Objects" {
            println!("Here is a list of Objects in our Mythological Dictionary:");
            // line break below, for space
            println!(" ");
            // Places Subcategory
            println!("Crystal Skulls, Beanstalk, Thor's Hammer");

            // ask the user to choose one name from the subcategory of Places
            let mut input = String::new();
            let mut results = Vec::new();

            println!("Enter One of the Objects to view a Description *Case Sensitive, for now*:");
            io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

            let input = input.trim();

            if input.is_empty() {
                results.push(String::from("Please enter a word or phrase to search for"));
            } else {
                results.push(input.to_string());
            }

            // return for subcategory selection by user
            if input.contains("Crystal Skulls") {
                println!("Crystal Skulls are the most spectacular objects in our Mythological Dictionary, besides the crown of ceoDemitri. Well To say forever Crystal Skulls is an Understatement. Superior.");
            } else if input.contains("Beanstalk") {
                println!("Beanstalk is the best object in our Mythological Dictionary. Well To say forever Beanstalk is an Innerstatement. Polo.");
            } else if input.contains("Thor's Hammer") {
                println!("Thor's Hammer is the most mysterious object in our Mythological Dictionary, besides the sword of ceoDemitri. Well To say forever America is an Understatement. Eyes.");
            } else { 
                println!("Sorry, we don't have that object in our Mythological Dictionary. Please try again.");
            }

            results.push("Objects".to_string());
        } else {
            println!("No results found for {}", input);
        }
    }

    results
}

// play again function starting from the introduction
fn play_again() {
    let mut play_again = String::new();

    println!(" ");
    println!("Would you like to play again?");
    io::stdin()
    .read_line(&mut play_again)
    .expect("Failed to read line");

    let play_again = play_again.trim();

    if play_again == "yes" || play_again == "y" {
        introduction();
        println!("\n");

        // manual coding for listing the categories. could be done in one of the functions above?  
        println!("Here are the categories of the Mythological Dictionary, Again.");
        println!("Please select a category from the list below to explore:");
        println!("\n");

        // maybe add a way in the future for an unbias search box for whatever topic and the results output accordingly
        list_categories();

        get_results(get_input());

    } else if play_again == "no" || play_again == "n" {
        println!("Thanks for playing!");
    } else {
        println!("Please enter either 'yes' or 'no'.");
    }


}




fn main() {
    introduction();
    
    println!("\n");
    // manual coding for listing the categories. could be done in one of the functions above?  
    println!("Here are the categories of the Mythological Dictionary.");
    println!("Please select a category from the list below to explore:");
    println!("\n");

    // maybe add a way in the future for an unbias search box for whatever topic and the results output accordingly
    list_categories();

    get_results(get_input());

    // make this loop so the user can play as many times as they want but exit the loop when the user chooses to quit
    // would you like to play again?




// THINGS To FIXXXXX
// capitalize user input for more flexibility
// make the game able to play multiple times
// Obviously, real dictionary items
// make the dictionary output better formatted
// update intro and outro
// extend the game to include more categories
// extend the game to include more rust uniqueness
// fix the input.contains to be more strict. instead user must be specific
