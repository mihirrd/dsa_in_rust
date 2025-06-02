mod linked_list;

fn main() {

    let v = vec![1, 2, 4];
    let head = linked_list::to_list(v);
    println!("{:?}",head);

}
