MODULE = rust/cpu

RUSTSRC := $(wildcard *.rs) $(BINDIR)/$(MODULE)/libc.rs
include $(RIOTBASE)/Makefile.base

$(BINDIR)/$(MODULE)/libc.cpprs: $(BINDIR)/$(MODULE)/
	$(Q)'$(RIOTRUST)/dist/tools/libc/libc.sh' > $@

$(BINDIR)/$(MODULE)/libc.rs: $(BINDIR)/$(MODULE)/libc.cpprs
	$(Q)$(CC) $(CFLAGS) $(INCLUDES) -E -x c $< | grep '^[^#]' > $@
