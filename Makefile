ASM_SRCS := $(wildcard *.s)
OBJDIR := build/obj
ASM_OBJS := $(patsubst %.s,$(OBJDIR)/%.o,$(ASM_SRCS))
MULTIBOOT_OBJ := $(OBJDIR)/multiboot_header.o

KERNEL_BIN := kernel.bin
ISO := kfc.iso
ISODIR := isodir
BOOTDIR := $(ISODIR)/boot
GRUBDIR := $(BOOTDIR)/grub
GRUB_CFG := $(GRUBDIR)/grub.cfg
RUST_LIB := target/i386-kfc-none/release/librust_kernel.a


.PHONY: all
all: help

.PHONY: help
help: ## Prints help for targets with comments
	@echo "- Use Rust nightly."
	@echo "- Required tools: grub (with grub-file), mtools, and QEMU 6.1 or newer."
	@echo "Available Rules:"
	@cat $(MAKEFILE_LIST) | grep -E '^[a-zA-Z_-]+:.*?## .*$$' | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'


.PHONY: asm
asm: $(ASM_OBJS)

.PHONY: kernel
kernel: $(KERNEL_BIN) ## Build the kernel executable

$(OBJDIR):
	mkdir -p $@

$(OBJDIR)/%.o: %.s | $(OBJDIR)
	nasm -f elf32 $< -o $@

$(RUST_LIB):
	cargo build --release

$(KERNEL_BIN): $(MULTIBOOT_OBJ) $(RUST_LIB) linker.ld
	ld -m elf_i386 -n -o $@ -T linker.ld $(ASM_OBJS) $(RUST_LIB)

$(BOOTDIR):
	mkdir -p $@

$(GRUBDIR):
	mkdir -p $@

$(BOOTDIR)/kernel.bin: $(KERNEL_BIN) | $(BOOTDIR)
	cp $< $@

$(GRUB_CFG): grub.cfg | $(GRUBDIR)
	cp $< $@

.PHONY: iso
iso: $(ISO) ## Build the kernel bootable iso

$(ISO): $(BOOTDIR)/kernel.bin $(GRUB_CFG)
	grub-mkrescue -o $@ $(ISODIR)

.PHONY: clean
clean: ## Clean all transient dependencies, delete the kernel executable and the ISO
	rm -rf $(OBJDIR)
	rm -f $(KERNEL_BIN) $(ISO)
	rm -f $(BOOTDIR)/kernel.bin

.PHONY: fclean
fclean: clean ## Clean all transient dependencies and delete the iso

.PHONY: run
run: $(ISO) ## Compile an run the ISO with qemu-system-1386
	qemu-system-i386 -cdrom $(ISO)
