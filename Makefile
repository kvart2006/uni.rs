arch ?= x86_64
target ?= $(arch)-unknown-uni.json
cargo_target ?= $(arch)-unknown-uni
test_target ?= $(arch)-unknown-linux-gnu

root = $(PWD)
src = $(root)/src

output_dir = $(root)/target

libuni = $(output_dir)/$(cargo_target)/release/libuni.rlib
libboot = $(output_dir)/$(cargo_target)/release/libboot.rlib
libboot_arch = $(output_dir)/$(cargo_target)/release/libboot_arch.a

libboot_arch_src = $(wildcard $(src)/libboot/src/arch/$(arch)/*.S)
libboot_arch_obj = $(libboot_arch_src:.S=.o)

AR = ar
RUSTC = rustc
CARGO = cargo

BIN_PATH ?= $(root)/examples/hello/main.rs
BIN_OUTPUT ?= hello

# -Wl is a dirty hack to pass libboot as linker argument
LDFLAGS = -n -nostdlib -static -T $(src)/libboot/src/arch/$(arch)/linker.ld -Wl,$(libboot)

RUSTC_FLAGS = --verbose --target $(target) --crate-type bin \
			  -L $(output_dir)/$(cargo_target)/release \
			  -L $(output_dir)/$(cargo_target)/release/deps \
			  -C link-args='$(LDFLAGS)' -l static=boot_arch

CARGO_FLAGS = --verbose --target $(target) --release

.PHONY: $(BIN_OBJ)

all: help

help:
	@echo '** Uni.rs build system **'
	@echo
	@echo 'This build system will ease you the task of building your'
	@echo 'application using the Uni.rs unikernel. In order to do so, a target'
	@echo 'called `bin` is responsible to build all Uni.rs libraries and link'
	@echo 'your code with it.'
	@echo
	@echo 'This target can be customized by 2 variables:'
	@echo '- BIN_PATH: path to your main .rs file'
	@echo '- BIN_OUTPUT: path to the output file'
	@echo
	@echo 'For example if you want to generate the hello binary from the'
	@echo 'example directory (examples/hello), you would do something like'
	@echo 'this:'
	@echo '`BIN_PATH=./examples/hello/main.rs BIN_OUTPUT=hello make bin`'
	@echo 'This will generate an hello binary in the current directory from the'
	@echo 'rust file named `examples/hello/main.rs`.'
	@echo
	@echo 'Other targets:'
	@echo -e 'test\t\t - Test uni.rs libraries'
	@echo -e 'runtime\t\t - Generate uni.rs libraries'
	@echo -e 'clean\t\t - Remove build temporary files'
	@echo -e 'distclean\t - Remove everything generated by the build system'

clean:
	rm -rf $(output_dir)
	rm -rf $(libboot_arch_obj)

distclean: clean
	rm -rf $(BIN_OUTPUT)

test:
	$(CARGO) test --target $(test_target) --verbose --manifest-path $(src)/libheap/Cargo.toml
	$(CARGO) test --target $(test_target) --verbose --manifest-path $(src)/libintrusive/Cargo.toml
	$(CARGO) test --target $(test_target) --verbose --manifest-path $(src)/libxen/Cargo.toml

bin: runtime
	$(RUSTC) $(RUSTC_FLAGS) $(BIN_PATH) -o $(BIN_OUTPUT)

runtime: libboot $(libboot_arch)

$(libboot_arch): $(libboot_arch_obj)
	ar csr $@ $^

libboot:
	CARGO_TARGET_DIR=$(output_dir) $(CARGO) rustc $(CARGO_FLAGS) --manifest-path $(src)/libboot/Cargo.toml
