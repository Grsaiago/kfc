.DEFAULT_GOAL := iso

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

.PHONY: all asm kernel iso clean help

all: iso

asm: $(ASM_OBJS)

kernel: $(KERNEL_BIN)

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

iso: $(ISO)

$(ISO): $(BOOTDIR)/kernel.bin $(GRUB_CFG)
	grub-mkrescue -o $@ $(ISODIR)

clean:
	rm -rf $(OBJDIR)
	rm -f $(KERNEL_BIN) $(ISO)
	rm -f $(BOOTDIR)/kernel.bin

run: $(ISO)
	qemu-system-i386 -cdrom $(ISO)

help:
	@echo "Use the Rust nightly toolchain."
	@echo "Required tools: grub (with grub-file), mtools, and QEMU 6.1 or newer."
