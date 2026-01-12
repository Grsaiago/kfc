rm -f isodir/boot/kernel.bin
rm -f kernel.bin
rm -f kfc.iso

cargo b --release
nasm -f elf32 multiboot_header.s

ld -m elf_i386 -n -o kernel.bin -T linker.ld multiboot_header.o target/i386-kfc-none/release/librust_kernel.a

cp kernel.bin isodir/boot/
grub-mkrescue -o kfc.iso isodir
