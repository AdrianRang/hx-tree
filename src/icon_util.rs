use crate::Element;

pub enum FILE_TYPE {
    DIRECTORY(bool),
    FILE(String)
}

pub fn get_icon(file_type: FILE_TYPE) -> String{
    String::from(
    match file_type {
        FILE_TYPE::DIRECTORY(open) => if open {""} else {""},
        FILE_TYPE::FILE(extension) => {
            match extension.as_str() {
                "rs" => "",
                "ts" => "",
                "toml" => "",
                "conf" => "",
                "gradle" => "",
                "json" => "",
                "md" => "󰍔",
                "lock" => "󰈡",
                "gitignore" => "󰊢",
                "tsx" => "󰜈",
                "java" => "",
                "js" => "",
                "html" => "",
                "css" => "",
                "env" => "",
                "c" => "",
                "cpp" => "󰙲",
                "csharp" => "󰌛",
                "go" => "󰟓",
                "py" => "󰌠",
                "rb" => "󰴭",
                "swift" => "󰛥",
                _ => ""
            }
        }
    })
} 

pub fn format_name(element: Element) -> String {
    let mut name = element.name.clone();
    let file_type;

    if element.is_directory {
        file_type = FILE_TYPE::DIRECTORY(element.is_expanded.get());
    } else {
        let extension = name.split(".").last().unwrap();

        file_type = FILE_TYPE::FILE(extension.to_string());
    }

    get_icon(file_type) + " " + name.as_str()
}
