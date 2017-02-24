# Name of your application.
APPLICATION = sock

# This has to be the absolute path to the RIOT base directory:
RIOTBASE ?= $(CURDIR)/../../..

# Change this to 0 show compiler invocation lines by default:
QUIET ?= 1

# The boards needs to support rust.
FEATURES_REQUIRED += rust_support

# If no BOARD is found in the environment, use this default:
BOARD ?= native

# SLIP cflags shit
ifeq (,$(SLIP_UART))
  # set default (last available UART)
  SLIP_UART="(UART_NUMOF-1)"
endif
ifeq (,$(SLIP_BAUDRATE))
  # set default
  SLIP_BAUDRATE=115200
endif
GNRC_NETIF_NUMOF := 2
INCLUDES += -I$(CURDIR)
CFLAGS += -DSLIP_UART=$(SLIP_UART)
CFLAGS += -DSLIP_BAUDRATE=$(SLIP_BAUDRATE)

# GNRC dependencies
USEMODULE += gnrc_netdev_default
USEMODULE += auto_init_gnrc_netif
# Specify the mandatory networking modules for IPv6 and UDP
USEMODULE += gnrc_ipv6_default
USEMODULE += gnrc_udp
USEMODULE += gnrc_slip
# Use sock with gnrc
USEMODULE += gnrc_sock_udp

# Depend on the rust/fmt wrapper for the fmt module.
USEMODULE += rust/fmt rust/xtimer rust/net

# include this for printing IP addresses
USEMODULE += shell_commands

include $(RIOTBASE)/Makefile.include
