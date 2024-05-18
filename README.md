<h1 align="center">
Clementine
</h1>

  
Clementine loader is a fresh take on and old technique.

  
### Description

The loader attempts to allocate the DLL at it's preferred address in order to avoid performing relocation. Afterwards it will copy the DLL's sections in the allocated memory and proceed with relocation if needed. The DOS and NT headers are left out, as having them in private memory doesn't make sense. Before changing the protection of each section, it will resolve imports and call the user-defined exported function (in this case `ClementineInit` 

The injector is planned to be simple, as it's purpose will be to show off the loader itself. 

The helper component is where I included the code I used to calculate the function names' hashes and print system addresses

### Features

- API hashing without walking the PEB

- As call stacks from LoadLibrary, NtAllocateVirtualMemory and NtProtectVirtualMemory leading to unbacked RX memory can lead to detections, Clementine uses `TpAllocWork` to create a custom call-stack, as per the second reference

- Some sort of sleep mask, after I decide on the technique

- x86 support is planned


### Credits

- Everything would have took longer to implement without: https://github.com/memN0ps/venom-rs/tree/main
- Custom call stacks: https://0xdarkvortex.dev/hiding-in-plainsight/
