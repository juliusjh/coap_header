pub mod coap_header;

pub use coap_header::{
    Header as CoAPHeader, MsgType as CoAPMsgType, RqCode as CoAPRqCode, Success as CoAPSuccess,
};

fn main() -> Result<(), &'static str> {
    let bytes: [u8; 4] = [64, 1, 0, 1];
    let header = CoAPHeader::from_bytes(bytes)?;
    println!("First:\n{}\n", header);

    let bytes1: [u8; 4] = [96, 95, 0, 1];
    let header1 = CoAPHeader::from_bytes(bytes1)?;
    println!("Second:\n{}\n", header1);

    let header2 = CoAPHeader::new(
        1,
        CoAPMsgType::Acknowlegment,
        0,
        CoAPRqCode::Success(CoAPSuccess::Created),
        1,
    );

    println!("Third :\n{}\n", header2);
    Ok(())
}
