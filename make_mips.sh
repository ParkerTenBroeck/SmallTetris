cargo build --release --target=./mips.json
cp ./target/mips/release/tetris ./mips/bin/com.o
mips-linux-gnu-objcopy -O binary -I elf32-tradbigmips ./mips/bin/com.o ./mips/bin/tmp.bin
mips-linux-gnu-objcopy -O binary --only-section .text --only-section .got --only-section .rodata --only-section .data -I elf32-tradbigmips --set-start 0x0 ./mips/bin/com.o ./mips/bin/static.o