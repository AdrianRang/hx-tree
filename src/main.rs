mod icon_util;

use std::{cell::Cell, process::Command};

use icon_util::*;

use ratatui::{DefaultTerminal, Frame, crossterm::event::{self, KeyCode}, layout::Constraint, style::{Color, Modifier}, widgets::{Block, Clear, List, ListState, Paragraph}};

const MAX_CHILDREN: usize = 8;

#[derive(Clone, Debug)]
struct Element {
    name: String,
    path: String,
    parent_element: Option<Box<Element>>,
    children: Vec<Element>,
    is_directory: bool,
    is_expanded: Cell<bool>
}

#[derive(Clone, Debug)]
struct TextInput {
    title: String,
    message: String,
    cursor: usize,
    enabled: bool
}

impl Element {
    fn root() -> Element {
        let output = Command::new("sh").arg("-c").arg("echo \"${PWD##*/}\"").output().expect(":(");
        let full_str = String::from_utf8(output.stdout).unwrap();
        
        Element { name: String::from("."), path: String::from("./"), parent_element: None, children: Vec::new(), is_directory: true, is_expanded: Cell::new(true)}
    }

    fn new(parent: Box<Element>, name: String) -> Element {
        return Element {
            name: name.clone(),
            path: parent.path.clone() + name.as_str(),
            parent_element: Some(parent),
            children: Vec::new(),
            is_directory: name.chars().last().unwrap() == '/',
            is_expanded: Cell::new(false)
        };
    }

    fn new_child(&mut self, name: String) -> Element {
        let child = Element::new(Box::new(self.clone()), name);        
        self.children.push(child.clone());
        
        child
    }

    fn find_child(&mut self, name: String) -> Option<&mut Element> {
        for child in &mut self.children {
            if child.is_directory {
                let mut chname = child.name.clone();
                chname.pop();
                if chname == name {
                    return Some(child)
                }
            } else {
                if child.name == name {
                    return Some(child);
                }
            }
        }

        None
    }

    fn populate_children(&mut self) {
        if !self.is_directory {return;}
       
        let output = Command::new("sh").arg("-c").arg("ls -a -F ".to_owned() + &self.path).output().expect(":(");
        let full_str = String::from_utf8(output.stdout).unwrap();
        let mut items: Vec<_> = full_str.split("\n").map(|s| s.to_string()).collect();
        items.remove(0);
        items.remove(0);
        items.pop();

        if items.len() >= MAX_CHILDREN {return;}

        //TODO: dont like that self.clone().
        items.iter().for_each(|i| self.children.push(Element::new(Box::new(self.clone()), i.clone())));
    }

    fn recursive_populate_children(&mut self) {
        if !self.is_directory {return;}
        
        self.populate_children();

        self.children.iter_mut().for_each(|f| f.recursive_populate_children());
    }

    fn _to_list(&self) -> Vec<String> {
        let mut out = Vec::new();
        self.children.iter().for_each(|f| out.push(f.path.clone()));

        return out;
    }

    fn to_indexed_list(root: &Element) -> Vec<&Element> {
        let mut list = Vec::new();

        root.children.iter().for_each(|c| {           
            list.push(c);

            if c.is_expanded.get() && c.is_directory {
                list.append(&mut Element::to_indexed_list(c));
            }
        });
        
        list
    }
    
    
    fn expand(&self) {
        if self.is_directory {
            self.is_expanded.set(!self.is_expanded.get());
        } else {
            open(self.clone());
        }
    }
 }

fn app(terminal: &mut DefaultTerminal, root: &mut Element) -> std::io::Result<()> {
    let mut list_state = ListState::default().with_selected(Some(0));

    let mut items = Element::to_indexed_list(root);

    let mut new_file = TextInput {
        title: String::from("New File"),
        message: String::new(),
        cursor: 0,
        enabled: false
    };

    terminal.draw(|frame| render(frame, items.iter().map(|i| format_name(i.clone().clone())).collect(), &mut list_state, new_file.clone()))?;

    loop {
    if let Some(key) = event::read()?.as_key_press_event() { 
        if !new_file.enabled {
            match key.code {
                KeyCode::Char('j') | KeyCode::Down => list_state.select_next(),
                KeyCode::Char('k') | KeyCode::Up => list_state.select_previous(),
                KeyCode::Char('q') | KeyCode::Esc => break Ok(()),
                KeyCode::Char(' ') | KeyCode::Right => {items.get(list_state.selected().unwrap()).unwrap().expand(); items = Element::to_indexed_list(root);},
                KeyCode::Char('n') => new_file.enabled = true,
                KeyCode::Char('r') => {root.children = Vec::new(); root.recursive_populate_children(); items = Element::to_indexed_list(root)},
                _ => {}
            }
        } else {
            match key.code {
                KeyCode::Char(to_insert) => new_file.message.push(to_insert),
                KeyCode::Backspace => { new_file.message.pop(); },
                KeyCode::Enter => {
                    let curr_item = items.get_mut(list_state.selected().unwrap()).unwrap();
                    if curr_item.is_directory {
                        curr_item.expand();
                        let mut_item = get_with_path(root, curr_item.path.clone());
                        open(mut_item.new_child(new_file.message));
                    } else {
                        let parent_path = curr_item.parent_element.as_ref().unwrap().clone().path;
                        let parent = get_with_path(root, parent_path);
                        open(parent.new_child(new_file.message));
                    }
                    items = Element::to_indexed_list(root);
                    new_file.enabled = false;
                    new_file.message = String::new();
                }
                KeyCode::Esc => {new_file.enabled = false; new_file.message = String::new();},
                _ => {}
            }
        }

        terminal.draw(|frame| render(frame, items.iter().map(|i| ("┆ ".to_owned().repeat(i.path.split("/").count() - if i.is_directory {3} else {2} )) + format_name(i.clone().clone()).as_str()).collect(), &mut list_state, new_file.clone()))?;
    }}
}

fn render(frame: &mut Frame, items: Vec<String>, list_state: &mut ListState, new_file: TextInput) {
    let list = List::new(items)
        .style(Color::White)
        .highlight_style(Modifier::REVERSED)
        .highlight_symbol("> ");
    
    frame.render_stateful_widget(list, frame.area(), list_state);

    if new_file.enabled {
        let popup_block = Block::bordered().title(new_file.title);
        let centered_area = frame.area().centered(Constraint::Percentage(90), Constraint::Percentage(10));

        frame.render_widget(Clear, centered_area);
        let paragraph = Paragraph::new(new_file.message).block(popup_block);
        frame.render_widget(paragraph, centered_area);
    }
}

fn main() {
    let mut root = Element::root();
    root.recursive_populate_children();

    println!("running TUI");

    ratatui::run(|t| app(t, &mut root)).unwrap();
}

fn get_with_path(root: &mut Element, path: String) -> Box<&mut Element> {
    let mut dir = path.split("/");
    let mut curr = root;
    dir.next();
    loop {
        if curr.path == path {
            return Box::new(curr);
        }
        match dir.next() {
            Some(name) => if name.len() > 0 {curr = curr.find_child(name.to_string()).unwrap() ;}
            None => return Box::new(curr),
        }
    }
}

fn open(file: Element) {
    if file.is_directory { return; }

    let path = file.path.replacen("./", "", 1);

     Command::new("tmux")
        .arg("send-keys")
        .arg("Escape")
    .output().unwrap();

    Command::new("tmux")
        .arg("send-keys")
        .arg(":open ".to_owned() + path.as_str())
        .arg("C-m")
    .output().unwrap();
}
