use std::fs::metadata;

#[cfg(test)]
mod tests;

pub fn is_regular_file(filename: &String) -> bool {
    let file_type = metadata(filename)
        .unwrap();

    file_type.is_file()
}
