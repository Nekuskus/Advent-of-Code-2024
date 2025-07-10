extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn get_input(_item: TokenStream) -> TokenStream {
    let span = proc_macro::Span::call_site();
    
    let path = span.file();

    let path_sep = std::path::MAIN_SEPARATOR_STR;
    let file_path_parts: Vec<&str> = path.split(path_sep).collect();

    // Take "year" from "crate_root/src/year/day.rs"
    let parent_name = file_path_parts.get(file_path_parts.len().saturating_sub(2)).unwrap();    
    
    // Take "crate_root" from "crate_root/lib/lib_root"
    let lib_root = env!("CARGO_MANIFEST_DIR");
    let crate_path_parts = lib_root.split(path_sep).collect::<Vec<&str>>();
    let crate_root = crate_path_parts[..crate_path_parts.len().saturating_sub(2)].to_owned().join(path_sep);

    let file_name = _item.to_string().chars().skip(1).take_while(|&c| c != '"').collect::<String>();

    format!(r##"include_str!(r#"{crate_root}{path_sep}inputs{path_sep}{parent_name}{path_sep}{file_name}"#).lines().map(|s| s.to_string()).collect()"##).parse().unwrap()
}
