use byteorder::LittleEndian;
use byteorder::ReadBytesExt;
use std::io::Cursor;

pub fn hashlittle(data: &[u8], init: u32) -> u32 {
    let mut size: u32 = data.len() as u32;
    let mut key = Cursor::new(data);

    let mut a: u32 = size.wrapping_add(init).wrapping_add(0xdeadbeefu32);
    let mut b: u32 = a;
    let mut c: u32 = a;

    while size > 12 {
        size -= 12;
        a = a.wrapping_add(key.read_u32::<LittleEndian>().unwrap());
        b = b.wrapping_add(key.read_u32::<LittleEndian>().unwrap());
        c = c.wrapping_add(key.read_u32::<LittleEndian>().unwrap());
        a = a.wrapping_sub(c); a ^= c.rotate_left(4); c = c.wrapping_add(b);
        b = b.wrapping_sub(a); b ^= a.rotate_left(6); a = a.wrapping_add(c);
        c = c.wrapping_sub(b); c ^= b.rotate_left(8); b = b.wrapping_add(a);
        a = a.wrapping_sub(c); a ^= c.rotate_left(16); c = c.wrapping_add(b);
        b = b.wrapping_sub(a); b ^= a.rotate_left(19); a = a.wrapping_add(c);
        c = c.wrapping_sub(b); c ^= b.rotate_left(4); b = b.wrapping_add(a);
    }

    if size > 4 {
        a = a.wrapping_add(key.read_u32::<LittleEndian>().unwrap());
    };
    if size > 8 {
        b = b.wrapping_add(key.read_u32::<LittleEndian>().unwrap());
    };
    match size {
        0 => {
            return c;
        }
        1 => {
            a = a.wrapping_add(key.read_u8().unwrap() as u32);
        }
        2 => {
            a = a.wrapping_add(key.read_u16::<LittleEndian>().unwrap() as u32);
        }
        3 => {
            a = a.wrapping_add(key.read_u24::<LittleEndian>().unwrap() as u32);
        }
        4 => {
            a = a.wrapping_add(key.read_u32::<LittleEndian>().unwrap());
        }
        5 => {
            b = b.wrapping_add(key.read_u8().unwrap() as u32);
        }
        6 => {
            b = b.wrapping_add(key.read_u16::<LittleEndian>().unwrap() as u32);
        }
        7 => {
            b = b.wrapping_add(key.read_u24::<LittleEndian>().unwrap() as u32);
        }
        8 => {
            b = b.wrapping_add(key.read_u32::<LittleEndian>().unwrap());
        }
        9 => {
            c = c.wrapping_add(key.read_u8().unwrap() as u32);
        }
        10 => {
            c = c.wrapping_add(key.read_u16::<LittleEndian>().unwrap() as u32);
        }
        11 => {
            c = c.wrapping_add(key.read_u24::<LittleEndian>().unwrap() as u32);
        }
        12 => {
            c = c.wrapping_add(key.read_u32::<LittleEndian>().unwrap());
        }
        _ => {
            unreachable!();
        }
    }

    c ^= b; c = c.wrapping_sub(b.rotate_left(14));
    a ^= c; a = a.wrapping_sub(c.rotate_left(11));
    b ^= a; b = b.wrapping_sub(a.rotate_left(25));
    c ^= b; c = c.wrapping_sub(b.rotate_left(16));
    a ^= c; a = a.wrapping_sub(c.rotate_left(4));
    b ^= a; b = b.wrapping_sub(a.rotate_left(14));
    c ^= b; c = c.wrapping_sub(b.rotate_left(24));

    c
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // As tested by the original implementation of hashlittle()
        let key = "Four score and seven years ago";

        let result = super::hashlittle(key.as_ref(), 0);
        assert_eq!(result, 0x17770551 as u32);

        let result = super::hashlittle(key.as_ref(), 1);
        assert_eq!(result, 0xcd628161 as u32);

    }
}
