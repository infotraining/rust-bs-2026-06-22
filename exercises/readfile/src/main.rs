mod custom_io;

fn main() {
    use custom_io::*;

    let file_content = read_file("file.txt");

    match file_content {
        Ok(content) => println!("File content: {}", content),
        Err(err) => println!("Error: {}", err),
    }

    let file_content = read_file("non_existent_file.txt");
    println!("File content: {:?}", file_content);

    let file_content = read_file("non_existent_file.txt");
    println!("File content: {:?}", file_content.unwrap_or_else(|err| {
        println!("Error reading file: {}", err);
        String::new()
    }));
}
