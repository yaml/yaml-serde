M := .cache/makes
$(shell [ -d $M ] || git clone -q https://github.com/makeplus/makes $M)
include $M/init.mk
include $M/rust.mk
include $M/yamlscript.mk
include $M/clean.mk
include $M/shell.mk

MAKES-CLEAN := \
  target \
  Cargo.lock \

CARGO-TARGETS := \
  build \
  check \
  clippy \
  install \
  test \
  uninstall \
  update \

RELEASE-UTIL := $(ROOT)/util/release

SECRETS-FILE := $(HOME)/.yaml-serde-secrets.yaml

RELEASE-STEPS := \
  $(CARGO) \
  release-check \
  release-bump \
  release-tag \
  release-publish \
  release-push \


$(CARGO-TARGETS): $(CARGO)
	cargo $@ $(opts)

release: $(RELEASE-STEPS)

release-publish: $(CARGO) $(YS)
	CARGO_TOKEN=$$(ys -e '.crates.token:say' $(SECRETS-FILE)) && \
	  $(CARGO) publish --token "$$CARGO_TOKEN"

release-check:
ifndef o
	@echo 'o=<old-version> required'
	@exit 1
endif
ifndef n
	@echo 'n=<new-version> required'
	@exit 1
endif
ifeq (,$(wildcard $(SECRETS-FILE)))
	@echo 'ERROR: $(SECRETS-FILE) not found' >&2
	@exit 1
endif
	NEW_VERSION=$(n) $(RELEASE-UTIL) release-check

release-bump:
	OLD_VERSION=$(o) NEW_VERSION=$(n) $(RELEASE-UTIL) version-bump

release-tag:
	$(RELEASE-UTIL) release-tag

release-push:
	$(RELEASE-UTIL) release-push
