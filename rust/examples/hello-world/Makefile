# Name of your application.
APPLICATION = hello_world

# This has to be the absolute path to the RIOT base directory:
RIOTBASE ?= $(CURDIR)/../../..

# Change this to 0 show compiler invocation lines by default:
QUIET ?= 1

# The boards needs to support rust.
FEATURES_REQUIRED += rust_support

# If no BOARD is found in the environment, use this default:
BOARD ?= native

# Depend on the rust/fmt wrapper for the fmt module.
USEMODULE += rust/fmt

include $(RIOTBASE)/Makefile.include
