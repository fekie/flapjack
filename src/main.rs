mod file_io;

fn main() {
    file_io::init_log_db();
    println!("{}", file_io::read_raw_db_contents().unwrap());
}
