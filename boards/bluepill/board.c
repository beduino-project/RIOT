/** TODO copyright comment */

/**
 * @ingroup     boards_bluepill
 * @{
 *
 * @file
 * @brief       Board specific implementations for the bluepill board
 *
 * @author      TODO
 *
 * @}
 */

#include "board.h"
#include "periph/gpio.h"

void board_init(void)
{
    cpu_init();
    gpio_init(LED0_PIN, GPIO_OUT);
}
