import binascii
import os

def htb(binary_program):
    return binascii.unhexlify(binary_program)

def write_to_disk(name, content):
    # create folder if it doesn't exist
    if not os.path.exists(f"roms/{name}"):
        os.makedirs(f"roms/{name}")

    # write to a file inside roms
    with open(f"roms/{name}/" + name.upper() + ".ch8", "wb") as f:
        f.write(content)

# insert name of the program
name = input("[ROM] Name: ")

# read from input from the terminal command
binary_program = input("[ROM] Binary: ")

# get content from the binary program (ASCII)
content = htb(binary_program)

# write to disk
write_to_disk(name, content)
