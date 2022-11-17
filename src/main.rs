/*
 * Write a script that can write a list of elements to a file
 * user will input elements
 */

mod list2File {
    public fn write_to_a_file(file_path: String, buf: String) -> std::io::Result<()>  {
        let mut fp = std::fs::File::create(file_path)?;
        fp.write_all(buf);
    }
    
} /* list2File */


fn main() {
    list2File::write_to_a_file("./data.txt", "This is a test");
    println!("Hello, world!");
}
