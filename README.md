# Clementine

Clementine loader is a fresh take on and old technique. The project is still WIP, so I would love PRs!

### Description

The loader attempts to allocate the DLL at it's preferred address in order to avoid performing relocation. Afterwards it will copy the DLL's sections in the allocated memory and proceed with relocation if needed. The DOS and NT headers are left out, as having them in private memory doesn't make sense. Before changing the protection of each section, it will resolve imports and call the user-defined exported function (in this case `ClementineInit` 

The injector is planned to be simple, as it's purpose will be to show off the loader itself

### Features

- As call stacks from LoadLibrary, NtAllocateVirtualMemory and NtProtectVirtualMemory leading to unbacked RX memory can lead to detections, Clementine uses `TpAllocWork` as per the second reference

- Sleeping via `KUSER_SHARED_DATA`

- x86 support is planned


### Credits

- Everything would have took longer to implement without: https://github.com/memN0ps/venom-rs/tree/main
- Custom call stacks: https://0xdarkvortex.dev/hiding-in-plainsight/
