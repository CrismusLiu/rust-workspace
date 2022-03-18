
use crate::platform::get_store;
use crate::platform::Store;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fmt::Display, ops::{Deref, DerefMut}};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TodoItem {
    pub id: u32,
    pub title: String, 
    pub completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct  Todos {
    todos: BTreeMap<u32, TodoItem>,
    next_id: u32
}

impl Default for Todos {
    fn default() -> Self {
        Self {
            todos: BTreeMap::new(),
            next_id: 1
        }
    }
}

impl Deref for Todos {
    type Target = BTreeMap<u32, TodoItem>;
    fn deref(&self) -> &Self::Target {
        &self.todos
    }
}

impl DerefMut for Todos {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.todos
    }
}

impl Display for Todos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        println!("{} => {}", &self.next_id, serde_json::to_string(&self).unwrap());
        Ok(())
    }
}

impl Todos {

    pub fn create_todo(&mut self, title: impl Into<String>) {
        let id = self.next_id;
        self.next_id += 1;
        self.insert(id, TodoItem {
            id,
            title: title.into(),
            completed: false
        });
        
        self.save();
    }

    pub fn show_clear_completed(&self) -> bool {
        self.iter().any(|(_, item)|{ item.completed })
    }

    pub fn clear_completed(&mut self) {
        self.retain(|_, todo| !todo.completed );
        self.save();
    }

    pub fn checkbox_toggle(&mut self, id: u32) {
        self.get_mut(&id).map(|todo| {                            
            todo.completed = !todo.completed;
        });
        self.save();
    }

    pub fn update_todo(&mut self, id: u32, title: impl Into<String>) {
        self.get_mut(&id).map(|todo| {
            todo.title = title.into();
        });
        self.save();
    }

    pub fn get_filtered_todos(&self, filter: &Filter) -> Vec<u32> {
        self.iter().filter(|(_, todo)| match filter {
            Filter::All => true,
            Filter::Active => !todo.completed,
            Filter::Completed => todo.completed
        })
        .map(|(id, _)| *id)
        .collect::<Vec<_>>()
    }

    fn save(&self) {
        let store = get_store();
        store.set(self);
    }


}

#[derive(Debug, Clone, PartialEq)]
pub enum Filter {
    All,
    Active,
    Completed
}

impl std::fmt::Display for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Filter::All => write!(f, "All"),
            Filter::Active => write!(f, "Active"),
            Filter::Completed => write!(f, "Completed"),        }    
    }

}

