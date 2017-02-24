#include <stdint.h>
#include "xtimer.h"

void
xsleep(uint32_t s)
{
	xtimer_sleep(s);
}

void
xusleep(uint32_t ms)
{
	xtimer_usleep(ms);
}

void
xnanosleep(uint32_t ns)
{
	xtimer_nanosleep(ns);
}
