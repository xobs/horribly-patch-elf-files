use anyhow::{anyhow, Result};
use std::env::args;
use std::fs::File;
use std::io::{Read, Write};

const EH_FRAME_HDR_START_MAGIC: u32 = 0x074f_72a8;
const EH_FRAME_HDR_END_MAGIC: u32 = 0xd15f_027a;
const EH_FRAME_START_MAGIC: u32 = 0x138f_dc0e;
const EH_FRAME_END_MAGIC: u32 = 0x8e41_1040;

fn patch_file(data: &mut [u8], magic: u32, value: u32) -> bool {
    let words = data.chunks_exact_mut(4);
    for word in words {
        let check_value = u32::from_le_bytes(word.try_into().unwrap());
        if check_value == magic {
            println!("Found magic value! {:08x} -> {:08x}", magic, value);
            for (dest, src) in word.iter_mut().zip(value.to_le_bytes().iter()) {
                *dest = *src;
            }
            return true;
        }
    }
    println!("Couldn't find magic");
    false
}

fn main() -> Result<()> {
    let target_filename = args()
        .nth(1)
        .ok_or(anyhow!("Please specify the filename"))?;

    let mut target_file = File::open(&target_filename)?;

    let mut target_data = vec![];
    target_file.read_to_end(&mut target_data)?;
    drop(target_file);

    let elf_target_data = target_data.clone();
    let elf_file = xmas_elf::ElfFile::new(&elf_target_data).or_else(|v| Err(anyhow!("{}", v)))?;

    if let Some(eh_frame_hdr) = elf_file.find_section_by_name(".eh_frame_hdr") {
        println!("eh_frame_hdr is: {:?}", eh_frame_hdr);
        patch_file(
            &mut target_data,
            EH_FRAME_HDR_START_MAGIC,
            eh_frame_hdr.address().try_into()?,
        );
        patch_file(
            &mut target_data,
            EH_FRAME_HDR_END_MAGIC,
            // (eh_frame_hdr.address() + eh_frame_hdr.size()).try_into()?,
            (eh_frame_hdr.size()).try_into()?,
        );
    } else {
        println!("Couldn't find eh_frame_hdr");
    }

    if let Some(eh_frame) = elf_file.find_section_by_name(".eh_frame") {
        println!("eh_frame is: {:?}", eh_frame);
        patch_file(
            &mut target_data,
            EH_FRAME_START_MAGIC,
            eh_frame.address().try_into()?,
        );
        patch_file(
            &mut target_data,
            EH_FRAME_END_MAGIC,
            // (eh_frame.address() + eh_frame.size()).try_into()?,
            (eh_frame.address()).try_into()?,
        );
    } else {
        println!("Couldn't find eh_frame");
    }

    let mut target_file = File::create(&target_filename)?;
    target_file.write_all(&mut target_data)?;

    Ok(())
}
