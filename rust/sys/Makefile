# Filter out rust modules and remove the 'rust/' prefix.
RSMODULES := $(patsubst rust/%,%,$(filter rust/%,$(USEMODULE)))

# Add the rust modules to DIRS.
DIRS += $(dir $(wildcard $(addsuffix /Makefile,$(RSMODULES))))

include $(RIOTBASE)/Makefile.base
