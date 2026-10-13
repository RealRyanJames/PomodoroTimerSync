struct CPacket {
    pub is_looping: bool,
    pub count: u8,
}

trait ChoicePacket {
    fn get_running_count(c: &mut CPacket) -> Self;
}

impl ChoicePacket for String {
    fn get_running_count(c: &mut CPacket) -> Self {
        c.count.to_string()
    }
}

pub fn setup_selection_packet() -> i32 {
    let mut c = CPacket {
        count: 1,
        is_looping: true,
    };

    if c.is_looping {
        String::get_running_count(&mut c);
    }

    c.count as i32
}
