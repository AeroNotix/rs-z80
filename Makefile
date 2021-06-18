ASM_DIR=src/roms
TEST_ASM_FILES:=$(wildcard ${ASM_DIR}/*.z80)
TEST_ROMS:=$(patsubst ${ASM_DIR}/%.z80,${ASM_DIR}/%.rom,${TEST_ASM_FILES})

${ASM_DIR}/%.rom: ${ASM_DIR}/%.z80
	zasm -l0 -i $^ -o $@

compile: ${TEST_ROMS}

test: compile
	cargo test
