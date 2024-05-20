<h1 align="center">
Clementine
</h1>

Clementine is a fresh take on reflective loading

# Description

The loader attempts to allocate the PE in chunks at it's preferred address in order to avoid performing relocation. Afterwards it will copy its sections in the allocated memory and proceed with relocation if needed. The DOS and NT headers are left out, as having them in private memory doesn't make sense. Before changing the protection of each section, it will resolve imports and  finish by calling the user-defined function (in this case `ClementineInit`)

The injector is planned to be simple, as it's purpose will be to show off the loader itself

The helper component contains the code I used to calculate the function names' hashes

### Features

- API hashing without walking the PEB. Since the injector will be backed by a file, it can safely call `LoadLibrary` and pass the address to the loader

- As call stacks from LoadLibrary, NtAllocateVirtualMemory and NtProtectVirtualMemory leading to unbacked RX memory can lead to detections, Clementine uses `TpAllocWork` to execute callbacks, as per the second reference

- Sets up exception handlers and executes TLS callbacks, if they exist

##### In Progress

- Sleep mask using ROP chains

- Inject PE in chunks and clean up after allocation

- Hooks

- x86 support

### Usage

Gonna fill this in when the project is done

 
# Credits

- Everything would have took longer to implement without: https://github.com/memN0ps/venom-rs/tree/main
- Custom call stacks: https://0xdarkvortex.dev/hiding-in-plainsight/
