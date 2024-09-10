use bevy_netcode::MyTrait;

#[derive(MyTrait)]
pub struct MyStruct;

fn main() {
    let instance = MyStruct;
    instance.my_function();
}
