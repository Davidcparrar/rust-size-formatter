#[derive(Debug)]
enum FileSize {
    Bytes(u64),
    Kilobytes(u64),
    Megabytes(u64),
    Gigabytes(u64),
}

impl FileSize {
    fn _format_size_template(&self,bytes: u64, kb: u64, mb : u64, gb: u64) -> String{
        format!(
            concat!(
                "{{",
                "bytes: \"{} bytes\", ",
                "kilobytes: \"{} kilobytes\", ",
                "megabytes: \"{} megabytes\", ",
                "gigabytes: \"{} gigabytes\"",
                "}}"), bytes, kb, mb, gb)
    }
    fn format_size(&self) -> String {

        match self {
            FileSize::Bytes(bytes) => self._format_size_template(
                *bytes, 
                bytes * 1000,
                bytes * 1000_000, 
                bytes * 1000_000_000
            ),
            FileSize::Kilobytes(kb) => self._format_size_template(
                kb *1000,
                *kb,
                kb / 1000, 
                kb / 1000_000
            ),            
            FileSize::Megabytes(mb) => self._format_size_template(
                mb * 1000_000,
                mb * 1000,
                *mb, 
                mb / 1000
            ), 
            FileSize::Gigabytes(gb) => self._format_size_template(
                gb * 1000_000_000,
                gb * 1000_000,
                gb * 1000, 
                *gb
            ), 
        }
    }
}
fn main() {
    // Read input arguments value (i64) size (str)
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Please provide two arguments: value (i64) and size (str)");
        return;
    }
    let parts: Vec<&str> = args[1].split_whitespace().collect();

    let value = parts[0].parse::<u64>().expect("Failed to parse value");
    let size = parts[1].to_string();

    let filesize = match size.as_str() {
        "bytes" => FileSize::Bytes(value),
        "kb" => FileSize::Kilobytes(value),
        "mb" => FileSize::Megabytes(value),
        "gb" => FileSize::Gigabytes(value),
        _ => {
            println!("Please provide a valid size (b, kb, mb, gb)");
            return;
        }
    };
    println!("String {}", filesize.format_size())
}
