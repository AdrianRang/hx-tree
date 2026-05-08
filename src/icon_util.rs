use crate::Element;

pub enum FileType {
    DIRECTORY(bool),
    FILE(String)
}

pub fn get_icon(file_type: FileType) -> String{
    String::from(
    match file_type {
        FileType::DIRECTORY(open) => if open {""} else {""},
        FileType::FILE(extension) => {
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
    let name = element.name.clone();
    let file_type;

    if element.is_directory {
        file_type = FileType::DIRECTORY(element.is_expanded.get());
    } else {
        let extension = name.split(".").last().unwrap();

        file_type = FileType::FILE(extension.to_string());
    }

    let with_icon = get_icon(file_type) + " " + name.as_str();
    
    ("┆ ".to_owned().repeat(element.path.split("/").count() - if element.is_directory {3} else {2} )) + &with_icon
}
