use ropey::Rope;

pub fn get_line_from_position(offset: usize, source_text: &str) -> Option<u32> {
    let rope = Rope::from_str(source_text);
    let line = rope.try_byte_to_line(offset).ok()? + 1;
    Some(line as u32)
}
