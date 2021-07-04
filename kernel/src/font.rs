use font::hankaku::HANKAKU_FONT;

pub fn get_font(x: char) -> [u8; 16] {
    let ascii_code = x as u8;
    HANKAKU_FONT[ascii_code as usize]
}