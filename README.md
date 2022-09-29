# meepy
This is a student project of mine and is in no shape or form an alternative to other mips32 
emulators. This project somewhat simulates cpu architecture and assembly file compilation,
first translating the assembly file into its binary encoding and storing it in memory behind 
32 bit addresses. Execution is performed by Rust. It also provides a small terminal gui that
prints the registers, current instruction, and the addresses of any stack, heap, or .data
memory (coming soon!)

brief summary of program:
- The program initialises the memory and registers, the addresses of the start of the labels
and gets all the instructions into an array.
- program::init() will call load\_file::read() and reads the file, sending the .text first
down a channel, then .data after.
- While the file is being read, what program::init() receives is sent to machine\_code::convert\_and\_store()
to be converted into its binary format (by calling to\_bin::convert() which requires op\_codes::get\_op\_and\_func())
It is to\_bin::convert() that sets the registers and immediate values etc into the binary format, 
get\_op\_and\_func() does what it says.
- program::init() returns with the all the instructions, returned by load\_file::read().
Unfortunately program::init() has to read the file twice to get the label addresses first
before we begin translating the instructions, because instructions like branch need an address
to branch to, which we don't have if we read and translate simultaneously, it just seemed more
fun to use multithreading :)
- We go back to main and call execute::x() which will execute the things in memory, beginning
at the address of the main label. We read the binary instruction and execute it by calling
binary\_ops::run(), which will disect the instruction by calling the corresponding instruction
(such as instructions::addiu()) with the parameters of the disected binary instruction into
its registers and/or immediate value (no offsets yet)
- We also print the registers and current executed instruction (ideas for a scrolling print
so we don't clear the instruction immediately)

Notes on any changed syntax for the asm.txt file
- there are two sections: .data, and .text, and any labels you wish to have
- not all commands will be implemented
- .data is only .word and .asciiz, which means I will just automatically align everything
  to a word.

