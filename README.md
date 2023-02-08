# Horrible Hack to ELF Files

This is a horrible hack. It replaces constants inside an ELF file so that you can use it to find its own unwind tables.

Normally you open your own ELF file and read it out, but not all operating systems have access to their own ELF file.

Other platforms manually add it using custom linker scripts, but not all platforms have that.

This is a middle ground for platforms that don't have another way to get debug symbols in.

## Usage

Add this to your ELF file, then run it through this program:

```rust
#[no_mangle]
pub static mut EH_FRM_HDR_OFFSET: usize = 0x074f_72a8;

#[no_mangle]
pub static EH_FRM_HDR_LEN: usize = 0xd15f_027a;

#[no_mangle]
pub static mut EH_FRM_OFFSET: usize = 0x138f_dc0e;

#[no_mangle]
pub static EH_FRM_LEN: usize = 0x8e41_1040;
```
