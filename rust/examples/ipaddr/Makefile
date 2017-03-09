# Name of your application.
APPLICATION = ipaddr

# This has to be the absolute path to the RIOT base directory:
RIOTBASE ?= $(CURDIR)/../../..

# Change this to 0 show compiler invocation lines by default:
QUIET ?= 1

# The boards needs to support rust.
FEATURES_REQUIRED += rust_support

# If no BOARD is found in the environment, use this default:
BOARD ?= native

# Depend on the rust/fmt wrapper for the fmt module.
USEMODULE += rust/fmt rust/net

USEMODULE += gnrc_ipv6_default
USEMODULE += ipv4_addr
USEMODULE += gnrc_sock_udp

include $(RIOTBASE)/Makefile.include
