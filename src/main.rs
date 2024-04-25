pub mod test_schema_capnp;

use capnp::{
    message::{Builder, ReaderOptions},
    serialize,
};
use test_schema_capnp::my_struct;

fn main() {
    let mut message = Builder::new_default();

    let mut my_data = message.init_root::<test_schema_capnp::my_struct::Builder>();
    my_data.set_id(666);
    my_data.set_descriptions("my very first test");

    let data_ser = serialize::write_message_segments_to_words(&message);
    println!("data_serialized={:?}", data_ser);

    let my_reader = serialize::read_message(data_ser.as_slice(), ReaderOptions::new()).unwrap();
    let my_data_read_back = my_reader.get_root::<my_struct::Reader>().unwrap();
    println!("my_data_read_back={:?}", my_data_read_back);
}
