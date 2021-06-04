.PHONY: build_all_versions check_all check $(MASTODON_VERSION_FEATURES_BUILD)

default: check_all

MASTODON_VERSION_FEATURES := $(shell cargo read-manifest | jq -r '.features | keys[]' | rg '^mastodon_')
MASTODON_VERSION_FEATURES_CHECK := $(foreach feature,$(MASTODON_VERSION_FEATURES), $(feature)_check)
MASTODON_VERSION_FEATURES_BUILD := $(foreach feature,$(MASTODON_VERSION_FEATURES), $(feature)_build)
MASTODON_VERSION_FEATURES_TEST  := $(foreach feature,$(MASTODON_VERSION_FEATURES), $(feature)_test)

check_all: check $(MASTODON_VERSION_FEATURES_CHECK)

build_all: $(MASTODON_VERSION_FEATURES_BUILD)

test_all: $(MASTODON_VERSION_FEATURES_TEST)

check: src/**/*.rs
	cargo check

$(MASTODON_VERSION_FEATURES_CHECK): src/**/*.rs
	cargo check --all-targets --no-default-features --features $(subst _check,,$@)

$(MASTODON_VERSION_FEATURES_BUILD): src/**/*.rs
	cargo build --no-default-features --features $(subst _build,,$@)

$(MASTODON_VERSION_FEATURES_TEST): src/**/*.rs
	cargo test --no-default-features --features $(subst _test,,$@)
