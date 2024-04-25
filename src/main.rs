use capnp::message::Builder;
use capnp::serialize;

pub mod test_schema_capnp;
use test_schema_capnp::my_struct;

fn main() {
    let mut message = Builder::new_default();

    let mut my_data = message.init_root::<test_schema_capnp::my_struct::Builder>();
    my_data.set_id(666);
    my_data.set_descriptions("my very first test");

    let data_ser = serialize::write_message_segments_to_words(&message);
    println!("{:?}", data_ser);
}
