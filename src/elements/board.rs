use super::list::List;

// Board of lists, broadest category
#[derive(Debug)]
pub struct Board {
    name: String,
    lists: Vec<List>
}

impl Board {
    // Constructor
    pub fn new(name: String) -> Board {
        Board {name, lists: vec![]}
    }

    // Getters and Setters
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_lists(&self) -> &Vec<List> {
        &self.lists
    }

    pub fn get_list(&mut self, index: usize) -> &mut List {
        if index > self.lists.len() {
            panic!("list index out of range");
        }
        &mut self.lists[index]
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn push_list(&mut self, list: List) {
        self.lists.push(list);
    }

    pub fn pop_list(&mut self) -> Option<List> {
        self.lists.pop()
    }

    pub fn remove_list(&mut self, index: usize) {
        self.lists.remove(index);
    }
}
