#[repr(C)]
pub enum gpio_mode {
    GPIO_IN,
    GPIO_IN_PD,
    GPIO_IN_PU,
    GPIO_OUT,
    GPIO_OD,
    GPIO_OD_PU,
}

#[repr(C)]
pub enum gpio_flank {
    GPIO_FALLING = 0,
    GPIO_RISING = 1,
    GPIO_BOTH = 2,
}
