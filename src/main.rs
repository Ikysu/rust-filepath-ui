use native_dialog::{FileDialog};

fn main() {
    match FileDialog::new()
    .add_filter("HTML", &["html"])
    .show_open_single_file() {
        Ok(pa)=>{
            match pa {
                Some(path) => {
                    print!("{}", path.display());
                },
                None => {
                    print!("NONE");
                }
            } 
        },
        Err(_)=>{
            print!("ERROR")
        }
    };
}
