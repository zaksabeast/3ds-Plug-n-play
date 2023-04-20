CRATE_NAME	= pnp_sys
TARGET_NAME	= pnp_sys
TITLE_ID = 0004013000CB9702
UNIQUE_ID = 0xCB97
PRODUCT_CODE = pnp

CARGO_BUILD_FLAGS = -Z build-std=core,alloc --target armv6k-nintendo-3ds

ifdef MODE3
	CARGO_BUILD_FLAGS	+= --features mode3
	TITLE_ID	= 0004013000CB9802
	UNIQUE_ID	= 0xCB98
	PRODUCT_CODE	= pnp_m3
	TARGET_NAME	= pnp_sys_mode3
endif

RUST_OUT_DIR	=	../target/armv6k-nintendo-3ds
RUST_RELEASE_DIR	=	$(RUST_OUT_DIR)/release
RUST_DEBUG_DIR	=	$(RUST_OUT_DIR)/debug

OUT_DIR	= out
RELEASE_DIR = $(OUT_DIR)/release
DEBUG_DIR = $(OUT_DIR)/debug

RELEASE_ELF	= $(RUST_RELEASE_DIR)/$(CRATE_NAME).elf
DEBUG_ELF	= $(RUST_DEBUG_DIR)/$(CRATE_NAME).elf

RELEASE_CXI	= $(RELEASE_DIR)/$(TITLE_ID).cxi
DEBUG_CXI	= $(DEBUG_DIR)/$(TITLE_ID).cxi

SOURCES = $(wildcard src/*.rs) $(wildcard src/**/*.rs) $(wildcard src/**/**/*.rs) $(wildcard src/**/**/**/*.rs) $(wildcard src/**/**/**/**/*.rs)

.PHONY: all clean test docs lint

all: release debug test

docs:
	@cargo +nightly doc --open $(CARGO_BUILD_FLAGS)

# Nightly and unstable options prevent clippy from linting dependencies - https://github.com/rust-lang/rust-clippy/issues/1066
lint:
	@cargo +nightly clippy -Z unstable-options $(CARGO_BUILD_FLAGS)

test:
	@cargo +nightly test

release: $(RELEASE_CXI)

debug: $(DEBUG_CXI)

$(RELEASE_ELF) : $(SOURCES)
	@cargo +nightly build --release $(CARGO_BUILD_FLAGS)

$(DEBUG_ELF) : $(SOURCES)
	@cargo +nightly build $(CARGO_BUILD_FLAGS)

$(RELEASE_CXI) : $(RELEASE_ELF)
	@mkdir -p $(RELEASE_DIR)
	@makerom -f ncch -rsf pnp.rsf -DPRODUCT_CODE=$(PRODUCT_CODE) -DUNIQUE_ID=$(UNIQUE_ID) -o $@ -elf $<
	@echo "Build release cxi"

$(DEBUG_CXI) : $(DEBUG_ELF)
	@mkdir -p $(DEBUG_DIR)
	@makerom -f ncch -rsf pnp.rsf -DPRODUCT_CODE=$(PRODUCT_CODE) -DUNIQUE_ID=$(UNIQUE_ID) -o $@ -elf $<
	@echo "Build debug cxi"

clean:
	@rm -rf $(OUT_DIR)
	@cargo clean
