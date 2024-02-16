use std::{collections::HashMap, u32, usize};

pub struct Storage{
    pub list: Vec<u32>,
    pub map_index: HashMap<u32,Vec<u32>>,
}

impl Storage{
    pub fn new() -> Self{
        Storage{
            list: Vec::new(),
            map_index: HashMap::new(),
        }
    }

    pub fn add(&mut self, value: u32){
        self.list.push(value);
        if self.map_index.contains_key(&value) {
           if let Some(list_index) = self.map_index.get_mut(&value){
                list_index.push((self.list.len()-1).try_into().unwrap());

           }
        } else {
            let mut vec_aux : Vec<u32> = Vec::new();
            vec_aux.push((self.list.len()-1) as u32);
            self.map_index.insert(value, vec_aux);
        }
    }

    pub fn del(&mut self,value: u32){
        if self.map_index.contains_key(&value) {
            if let Some(list_index) = self.map_index.get_mut(&value){
                if let Some(last_value) = self.list.last() {
                    if *last_value == value {
                        self.list.pop();
                    }
                }
                for index in list_index{
                    let size = self.list.len()-1;
                    if index <= &mut (size as u32) {
                        let aux = self.list[*index as usize];
                        self.list[*index as usize] = self.list[size];
                        self.list[size] = aux;
                        self.list.pop();
                    }
                }
            }
            self.map_index.remove(&value);
        }
    }
}
