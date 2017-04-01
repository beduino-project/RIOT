# Auto generate an ffi.rs file from a C header.
RUST_FFI_HEADER = ffi.h

# Types from the headers to autogenerate bindings for.
RUST_FFI_TYPES = gpio_t gpio_mode_t gpio_isr_ctx_t \
	pwm_t pwm_mode_t

# GPIO Functions to autogenerate bindings for.
RUST_FFI_FUNCS = gpio_init gpio_read gpio_write gpio_toggle \
	pwm_init pwm_channels pwm_set pwm_poweron pwm_poweroff

# Build libffi.a to wrap C Macros.
RUST_FFI = 1

include $(RIOTRUST)/Makefile.base
