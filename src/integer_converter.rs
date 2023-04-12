//! convert integer

macro_rules! clip {
    ($from_t:ty, $to_t:ty, $n:expr) => (
        if $n > <$to_t>::MAX as $from_t {
            <$to_t>::MAX
        } else if $n < <$to_t>::MIN as $from_t {
            <$to_t>::MIN
        } else {
            let ret: $to_t = $n.try_into().unwrap();
            ret
        }
    )
}


// isize

pub fn i_to_i8(a: isize) -> i8 {
    clip!(isize, i8, a)
}

pub fn i_to_u8(a: isize) -> u8 {
    clip!(isize, u8, a)
}

pub fn i_to_i16(a: isize) -> i16 {
    clip!(isize, i16, a)
}

pub fn i_to_u16(a: isize) -> u16 {
    clip!(isize, u16, a)
}

pub fn i_to_i32(a: isize) -> i32 {
    clip!(isize, i32, a)
}

pub fn i_to_u32(a: isize) -> u32 {
    clip!(isize, u32, a)
}

pub fn i_to_i64(a: isize) -> i64 {
    clip!(isize, i64, a)
}

pub fn i_to_u64(a: isize) -> u64 {
    clip!(isize, u64, a)
}


// usize

pub fn u_to_i8(a: usize) -> i8 {
    clip!(usize, i8, a)
}

pub fn u_to_u8(a: usize) -> u8 {
    clip!(usize, u8, a)
}

pub fn u_to_i16(a: usize) -> i16 {
    clip!(usize, i16, a)
}

pub fn u_to_u16(a: usize) -> u16 {
    clip!(usize, u16, a)
}

pub fn u_to_i32(a: usize) -> i32 {
    clip!(usize, i32, a)
}

pub fn u_to_u32(a: usize) -> u32 {
    clip!(usize, u32, a)
}

pub fn u_to_i64(a: usize) -> i64 {
    clip!(usize, i64, a)
}

pub fn u_to_u64(a: usize) -> u64 {
    clip!(usize, u64, a)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i8_1() {
        assert_eq!(i_to_i8(32isize), 32);
    }

    #[test]
    fn test_i8_2() {
        assert_eq!(i_to_i8(-32isize), -32);
    }

    #[test]
    fn test_i8_3() {
        assert_eq!(i_to_i8(-332isize), i8::MIN);
    }

    #[test]
    fn test_i8_4() {
        assert_eq!(i_to_i8(332isize), i8::MAX);
    }

    #[test]
    fn test_u8_1() {
        assert_eq!(i_to_u8(32isize), 32);
    }

    #[test]
    fn test_u8_2() {
        assert_eq!(i_to_u8(-32isize), 0);
    }

    #[test]
    fn test_u8_3() {
        assert_eq!(i_to_u8(-332isize), u8::MIN);
    }

    #[test]
    fn test_u8_4() {
        assert_eq!(i_to_u8(332isize), u8::MAX);
    }
}
