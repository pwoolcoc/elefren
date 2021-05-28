.PHONY: book build_all_versions $(MASTODON_VERSION_FEATURES_BUILD)

default: check_all

MASTODON_VERSION_FEATURES := mastodon_1_5_0 \
		mastodon_2_1_0 \
		mastodon_2_1_2 \
		mastodon_2_2_0 \
		mastodon_2_4_0 \
		mastodon_2_9_1 \
		mastodon_3_0_0 \
		mastodon_3_1_0 \
		mastodon_3_3_0

MASTODON_VERSION_FEATURES_CHECK := $(foreach feature,$(MASTODON_VERSION_FEATURES), $(feature)_check)
MASTODON_VERSION_FEATURES_BUILD := $(foreach feature,$(MASTODON_VERSION_FEATURES), $(feature)_build)
MASTODON_VERSION_FEATURES_TEST  := $(foreach feature,$(MASTODON_VERSION_FEATURES), $(feature)_test)

check_all: $(MASTODON_VERSION_FEATURES_CHECK)

build_all: $(MASTODON_VERSION_FEATURES_BUILD)

test_all: $(MASTODON_VERSION_FEATURES_TEST)

$(MASTODON_VERSION_FEATURES_CHECK): src/**/*.rs
	cargo check --all-targets --no-default-features --features $(subst _check,,$@)

$(MASTODON_VERSION_FEATURES_BUILD): src/**/*.rs
	cargo build --no-default-features --features $(subst _build,,$@)

$(MASTODON_VERSION_FEATURES_TEST): src/**/*.rs
	cargo test --no-default-features --features $(subst _test,,$@)
