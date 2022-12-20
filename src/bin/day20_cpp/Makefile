.PHONY: all clean
.SILENT: clean

CC=g++
CFLAGS=@./compile_flags.txt

all:
	@echo Please enter a target name
	@exit 1

%: %.cpp
	$(CC) $< $(CFLAGS) -o $@
clean:
	fd -tx -xrm
	rm -rf *.dSYM

