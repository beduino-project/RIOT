extern crate basecpu;

macro_rules! mode {
    ($x:expr, $y:expr, $z:expr) => {
        ($x | ($y << 2) | ($z << 4))
    };
}

#[repr(C)]
pub enum gpio_mode {
    GPIO_IN    = mode!(0, 1, 0),
    GPIO_IN_PD = mode!(0, 2, 0),
    GPIO_IN_PU = mode!(0, 2, 1),
    GPIO_OUT   = mode!(3, 0, 0),
    GPIO_OD    = mode!(3, 1, 0),
    GPIO_OD_PU = (0xff),
}

pub type gpio_flank = basecpu::periph_cpu::gpio_flank;
