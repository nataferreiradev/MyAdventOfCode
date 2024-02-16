mod storage;
use storage::Storage;

fn main() {
    let mut storage: Storage = Storage::new();

    storage.add(1);
    storage.add(2);
    storage.add(1);
    storage.add(3);
    storage.add(1);
    storage.add(2);
    storage.add(1);
    storage.add(2);
    storage.add(4);
    storage.add(3);


    
    println!("=========");
    println!("{:?}",storage.map_index);
    println!("{:?}",storage.list);
    println!("=========");
    storage.del(3);
    println!("{:?}",storage.map_index);
    println!("{:?}",storage.list);
    println!("=========");
}
