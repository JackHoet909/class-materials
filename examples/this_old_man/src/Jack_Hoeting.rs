fn main() {
    //description of each location
    let on_my_locations = 
    [
        "on my thumb", "on my shoe", "on my knee", "on my door", "on my hive", 
        "on my sticks", "up in heaven", "on my gate", "on my spine", "once again"
    ];

//Enumerate() was called to snatch the index of the location along with the value of the number
//iterates through the array with a for loop
    for (i, location) in on_my_locations.iter().enumerate()
    {
        let number = i + 1; //1-based array
        print_each_verse(location, number); //calls this function to print every verse
    }
}

//function to print each verse of the song depending the number and location
fn print_each_verse(location: &str, number: usize) 
{
    println!("This old man, he played {},", number);
    println!("He played knick-knack {};", location);
    println!("With a knick-knack paddywhack,");
    println!("Give the dog a bone,");
    println!("This old man came rolling home.\n"); // \n creates a space for each verse
}



/* Lyrics to This Old Man

This old man, he played one,
He played knick-knack on my thumb;
With a knick-knack paddywhack,
Give the dog a bone,
This old man came rolling home.

This old man, he played two,
He played knick-knack on my shoe;
With a knick-knack paddywhack,
Give the dog a bone,
This old man came rolling home.

This old man, he played three,
He played knick-knack on my knee;
With a knick-knack paddywhack,
Give the dog a bone,
This old man came rolling home.

This old man, he played four,
He played knick-knack on my door;
With a knick-knack paddywhack,
Give the dog a bone,
This old man came rolling home.

This old man, he played five,
He played knick-knack on my hive;
With a knick-knack paddywhack,
Give the dog a bone,
This old man came rolling home.

This old man, he played six,
He played knick-knack on my sticks;
With a knick-knack paddywhack,
Give the dog a bone,
This old man came rolling home.

This old man, he played seven,
He played knick-knack up in heaven;
With a knick-knack paddywhack,
Give the dog a bone,
This old man came rolling home.

This old man, he played eight,
He played knick-knack on my gate;
With a knick-knack paddywhack,
Give the dog a bone,
This old man came rolling home.

This old man, he played nine,
He played knick-knack on my spine;
With a knick-knack paddywhack,
Give the dog a bone,
This old man came rolling home.

This old man, he played ten,
He played knick-knack once again;
With a knick-knack paddywhack,
Give the dog a bone,
This old man came rolling home.
*/