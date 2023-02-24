use crate::register::register::Register;

pub trait BitField: Into<u8> + Copy{
    fn start(&self) -> u8;
    fn end(&self) -> u8;
    fn register(&self) -> Register;
    fn len(&self) -> u8 {
        self.end() - self.start() + 1
    }
    fn mask(&self) -> u8 {
        (((1 << (self.len())) - 1) as u8) << self.start()
    }
}


#[macro_export]
macro_rules! impl_bitfield {
    ($t:ty, $start:expr, $end:expr, $register:expr) => {
        impl BitField for $t {
            fn start(&self) -> u8 {$start}
            fn end(&self) -> u8 {$end}
            fn register(&self) -> Register {$register}
        }

        impl From<$t> for u8 {
            fn from(a: $t) -> u8 {
                a as u8
            }
        }
    };
}
