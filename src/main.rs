use clap::Parser;
use object::{Object, ObjectSection};

#[derive(Parser)]
#[clap(about, version, author)] // Pull these from `Cargo.toml`
struct Cli {
    /// Analyze binary
    #[clap(default_value = "a.out", value_name = "BIN")]
    bin: std::path::PathBuf,
}

#[repr(C)]
struct Elf64Dyn {
    d_tag: Elf64Sxword,
    d_val: Elf64Word,
    // d_ptr: Elf64Addr,
}

type Elf64Addr = u64;
type Elf64Word = u32;
type Elf64Sxword = i64;
type Elf64Xword = u64;

pub const DT_NULL: Elf64Sxword = 0x00;
pub const DT_NEEDED: Elf64Sxword = 0x00000001;
pub const DT_PLTRELSZ: Elf64Sxword = 0x2;
pub const DT_PLTGOT: Elf64Sxword = 0x03;
pub const DT_HASH: Elf64Sxword = 0x04;
pub const DT_STRTAB: Elf64Sxword = 0x05;
pub const DT_SYMTAB: Elf64Sxword = 0x06;
pub const DT_RELA: Elf64Sxword = 0x07;
pub const DT_RELASZ: Elf64Sxword = 0x08;
pub const DT_RELAENT: Elf64Sxword = 0x09;
pub const DT_STRSZ: Elf64Sxword = 0x0a;
pub const DT_SYMENT: Elf64Sxword = 0x0b;
pub const DT_INIT: Elf64Sxword = 0x0c;
pub const DT_FINI: Elf64Sxword = 0x0d;
pub const DT_SONAME: Elf64Sxword = 0x0e;
pub const DT_RPATH: Elf64Sxword = 0x0f;
pub const DT_SYMBOLIC: Elf64Sxword = 0x10;
pub const DT_REL: Elf64Sxword = 0x11;
pub const DT_RELSZ: Elf64Sxword = 0x12;
pub const DT_RELENT: Elf64Sxword = 0x13;
pub const DT_PLTREL: Elf64Sxword = 0x14;
pub const DT_DEBUG: Elf64Sxword = 0x15;
pub const DT_TEXTREL: Elf64Sxword = 0x16;
pub const DT_JMPREL: Elf64Sxword = 0x17;
pub const DT_BIND_NOW: Elf64Sxword = 0x18;
pub const DT_INIT_ARRAY: Elf64Sxword = 0x19;
pub const DT_FINI_ARRAY: Elf64Sxword = 0x1a;
pub const DT_INIT_ARRAYSZ: Elf64Sxword = 0x1b;
pub const DT_FINI_ARRAYSZ: Elf64Sxword = 0x1c;
pub const DT_RUNPATH: Elf64Sxword = 0x1d;
pub const DT_FLAGS: Elf64Sxword = 0x1e;
pub const DT_ENCODING: Elf64Sxword = 0x1f;
pub const DT_PREINIT_ARRAY: Elf64Sxword = 0x20;
pub const DT_PREINIT_ARRAYSZ: Elf64Sxword = 0x21;
pub const DT_SYMTAB_SHNDX: Elf64Sxword = 0x22;
pub const DT_LOOS: Elf64Sxword = 0x6000_000d;
pub const DT_HIOS: Elf64Sxword = 0x6fff_f000;
pub const DT_LOPROC: Elf64Sxword = 0x7000_0000;
pub const DT_HIPROC: Elf64Sxword = 0x7fff_ffff;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let bin = std::fs::read(&args.bin)?;
    let elf = object::File::parse(&*bin)?;

    if let Some(section) = elf.section_by_name(".dynamic") {
        let mut bytes: i64 = 0;
        for (i, data) in section.data()?.iter().enumerate() {
            if i % 8 == 0 && i != 0 {
                match (i, bytes) {
                    (p, _) if p % 16 == 0 => print!("0x{:08x?} ", bytes),
                    (p, DT_NEEDED) if p % 16 == 8 => print!("NEEDED(0x{:08x?})\t", bytes),
                    // TODO
                    _ => print!("UNKNOWN(0x{:08x?})\t", bytes),
                }
                bytes = 0x00000000;
            }

            if i % 16 == 0 && i != 0 {
                println!("");
            }
            bytes |= ((*data as i64) << 8 * (i % 4)) as i64;
        }
    } else {
        println!("section not available");
    }

    Ok(())
}
