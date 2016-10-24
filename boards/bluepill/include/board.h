/*
 * Copyright (C) 2015 TriaGnoSys GmbH
 *
 * This file is subject to the terms and conditions of the GNU Lesser
 * General Public License v2.1. See the file LICENSE in the top level
 * directory for more details.
 */

/**
 * @defgroup    boards_nucleo-f103 Nucleo-F103
 * @ingroup     boards
 * @brief       Board specific files for the nucleo-f103 board
 * @{
 *
 * @file
 * @brief       Board specific definitions for the nucleo-f103 board
 *
 * @author      Víctor Ariño <victor.arino@triagnosys.com>
 */

#ifndef BOARD_H_
#define BOARD_H_

#include "periph_conf.h"
#include "periph/spi.h"

#ifdef __cplusplus
extern "C" {
#endif

/**
 * @brief   LED pin definitions and handlers
 * @{
 */
#define LED0_PIN            GPIO_PIN(PORT_C , 13)

#define LED0_MASK           (1 << 13)

#if defined(CPU_FAM_STM32F4)
#define LED_CREG            BSRRH
#else
#define LED_CREG            BRR
#endif
#if defined(CPU_FAM_STM32F3) || defined(CPU_FAM_STM32F4) || defined(CPU_FAM_STM32L1)
#define LED_SREG            BSRRL
#else
#define LED_SREG            BSRR
#endif

#define LED0_ON             (GPIOC->LED_SREG = LED0_MASK)
#define LED0_OFF            (GPIOC->LED_CREG = LED0_MASK)
#define LED0_TOGGLE         (GPIOC->ODR     ^= LED0_MASK)
/** @} */

/**
 * @brief   SPI definitions for Openlabs board
 * @{
 */
#define AT86RF2XX_PARAMS_BOARD      {.spi = SPI_0, \
                                     .spi_speed = SPI_SPEED_5MHZ, \
                                     .cs_pin = GPIO_PIN(PORT_A, 4), \
                                     .int_pin = GPIO_PIN(PORT_B, 0), \
                                     .sleep_pin = GPIO_PIN(PORT_B, 1), \
                                     .reset_pin = GPIO_PIN(PORT_A, 3)}
/** @} */

/**
 * @brief   User button
 */
#define BTN_B1_PIN          GPIO_PIN(PORT_C, 13)

/**
 * @brief   Initialize board specific hardware, including clock, LEDs and std-IO
 */
void board_init(void);

/**
 * @brief Use the 2nd UART for STDIO on this board
 */
#define UART_STDIO_DEV      UART_DEV(1)

/**
 * @name xtimer configuration
 */
#define XTIMER_WIDTH        (16)
#define XTIMER_BACKOFF      5
/** @} */

#ifdef __cplusplus
}
#endif

#endif /* BOARD_H_ */
/** @} */
