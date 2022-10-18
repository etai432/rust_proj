pub fn run() {
    //used to check the condition of something and act on the result
    let age: u8 = 18;
    let check_id: bool = false;
    let knows_person_of_age = true;
    //If/Else
    if age >= 21 && check_id || knows_person_of_age{
        println!("Bartender: what would you like to drink?");
    }
    else if age < 21 && check_id{
        println!("Bartender: sorry, you have to leave");
    }
    else {
        println!("Bartender: ill need to see your ID");
    }

    //Shorthand If
    let is_of_age = if age >= 21 {true} else {false};
    println!("is of age: {}", is_of_age);
}