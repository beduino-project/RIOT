MODULE = rust/gcoap

COAP_FLAGS = COAP_GET COAP_POST COAP_PUT COAP_DELETE
COAP_FORMATS = COAP_FORMAT_TEXT COAP_FORMAT_LINK COAP_FORMAT_OCTET \
	      COAP_FORMAT_JSON COAP_FORMAT_CBOR COAP_FORMAT_NONE

# Response message codes: success
COAP_CODES := COAP_CODE_CREATED COAP_CODE_DELETED COAP_CODE_VALID \
	     COAP_CODE_CHANGED COAP_CODE_CONTENT COAP_CODE_205

# Response message codes: client error
COAP_CODES += COAP_CODE_BAD_REQUEST COAP_CODE_UNAUTHORIZED \
	      COAP_CODE_BAD_OPTION COAP_CODE_FORBIDDEN \
	      COAP_CODE_PATH_NOT_FOUND COAP_CODE_404 \
	      COAP_CODE_METHOD_NOT_ALLOWED \
	      COAP_CODE_NOT_ACCEPTABLE \
	      COAP_CODE_PRECONDITION_FAILED \
	      COAP_CODE_REQUEST_ENTITY_TOO_LARGE \
	      COAP_CODE_UNSUPPORTED_CONTENT_FORMAT

# Response message codes: server error
COAP_CODES += COAP_CODE_INTERNAL_SERVER_ERROR \
	      COAP_CODE_NOT_IMPLEMENTED \
	      COAP_CODE_BAD_GATEWAY \
	      COAP_CODE_SERVICE_UNAVAILABLE \
	      COAP_CODE_GATEWAY_TIMEOUT \
	      COAP_CODE_PROXYING_NOT_SUPPORTED

RUST_FFI_HEADER = $(RIOTBASE)/sys/include/net/gcoap.h
RUST_FFI_FUNCS  = gcoap_register_listener gcoap_resp_init gcoap_finish
RUST_FFI_TYPES  = coap_pkt_t gcoap_listener_t coap_resource_t
RUST_FFI_VARS   = $(COAP_FLAGS) $(COAP_CODES) $(COAP_FORMATS)

RUSTSRC := $(wildcard *.rs) \
	$(BINDIR)/$(MODULE)/codes.rs \
	$(BINDIR)/$(MODULE)/flags.rs \
	$(BINDIR)/$(MODULE)/formats.rs
include $(RIOTBASE)/Makefile.base

$(BINDIR)/$(MODULE)/codes.rs: $(BINDIR)/$(MODULE)/
	$(Q)printf "\n" > $@
	$(Q)for code in $(COAP_CODES); do \
		echo "pub use ::ffi::$$code as $$(echo $$code | cut -d _ -f2-);" >> $@; \
	done

$(BINDIR)/$(MODULE)/flags.rs: $(BINDIR)/$(MODULE)/
	$(Q)printf "\n" > $@
	$(Q)for flag in $(COAP_FLAGS); do \
		echo "pub use ::ffi::$$flag as $$(echo $$flag | cut -d _ -f2-);" >> $@; \
	done

$(BINDIR)/$(MODULE)/formats.rs: $(BINDIR)/$(MODULE)/
	$(Q)printf "\n" > $@
	$(Q)for format in $(COAP_FORMATS); do \
		echo "pub use ::ffi::$$format as $$(echo $$format | cut -d _ -f3-);" >> $@; \
	done
