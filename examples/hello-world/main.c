/*
 * Copyright (C) 2014 Freie Universität Berlin
 *
 * This file is subject to the terms and conditions of the GNU Lesser
 * General Public License v2.1. See the file LICENSE in the top level
 * directory for more details.
 */

/**
 * @ingroup     examples
 * @{
 *
 * @file
 * @brief       Hello World application
 *
 * @author      Kaspar Schleiser <kaspar@schleiser.de>
 * @author      Ludwig Knüpfer <ludwig.knuepfer@fu-berlin.de>
 *
 * @}
 */

#include <stdio.h>

#include <stdint.h>
#include "cpu_conf.h"
#include "periph_conf.h"

int main(void)
{
    // we need two 32 bit words for profiling
    uint32_t profA, profB;

    // enable the cycle counter (needed only once)
    DWT->CTRL |= DWT_CTRL_CYCCNTENA_Msk;

    // start profiling
    profA = DWT->CYCCNT;

    puts("Hello World!");

    printf("You are running RIOT on a(n) %s board.\n", RIOT_BOARD);
    printf("This board features a(n) %s MCU.\n", RIOT_MCU);

    // end profiling
    profB = DWT->CYCCNT;

    // interpret result
    // (This assumes that the counter did not overflow, which is not
    // generally a valid assumption, so take result with grain of salt)
    profA = profB - profA;
    profB = profA / (CLOCK_CORECLOCK / 1000000);
    printf("PROF %lu ticks, %lu usecs\n", profA, profB);

    return 0;
}
