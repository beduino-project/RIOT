#include "periph/gpio.h"
#include "periph/pwm.h"

unsigned int
gpio_pin(int x, int y)
{
	return GPIO_PIN(x, y);
}

pwm_t
pwm_dev(int x)
{
	return PWM_DEV(x);
}
