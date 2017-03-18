# The `rust/core` Makefile pulls in `rust/cpu`
DIRS += $(RIOTRUST)/core

# Filter out rust modules and remove the `rust/` prefix.
RSMODULES := $(patsubst rust/%,%,$(filter rust/%,$(USEMODULE)))

# Add modules in `drivers/` and `sys/` to `DIRS`.
DIRS += $(dir $(wildcard $(addprefix drivers/,$(addsuffix /Makefile,$(RSMODULES)))))
DIRS += $(dir $(wildcard $(addprefix sys/,$(addsuffix /Makefile,$(RSMODULES)))))

include $(RIOTBASE)/Makefile.base
