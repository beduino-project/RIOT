#include "periph/gpio.h"

unsigned int
gpio_pin(int x, int y)
{
	return GPIO_PIN(x, y);
}
