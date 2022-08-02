use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

fn main() {
    let vfct = std::env::args().nth(1).unwrap();
    let output = std::env::args().nth(2).unwrap();
    let vfct = std::fs::read(vfct).unwrap();
    let mut cursor = std::io::Cursor::new(vfct);
    println!("Table:");
    println!("\tSignature: {}", {
        let mut buf = [0; 4];
        cursor.read_exact(&mut buf).unwrap();
        core::str::from_utf8(&buf).unwrap().to_owned()
    });
    println!(
        "\tLength: {:#X?}",
        cursor.read_u32::<LittleEndian>().unwrap()
    );
    println!("\tRevision: {:#X?}", cursor.read_u8().unwrap());
    println!("\tChecksum: {:#X?}", cursor.read_u8().unwrap());
    println!("\tOEM ID: {}", {
        let mut buf = [0; 6];
        cursor.read_exact(&mut buf).unwrap();
        core::str::from_utf8(&buf).unwrap().to_owned()
    });
    println!("\tOEM Table ID: {}", {
        let mut buf = [0; 8];
        cursor.read_exact(&mut buf).unwrap();
        core::str::from_utf8(&buf).unwrap().to_owned()
    });
    println!(
        "\tOEM Revision: {:#X?}",
        cursor.read_u32::<LittleEndian>().unwrap()
    );
    println!("\tCreator ID: {}", {
        let mut buf = [0; 4];
        cursor.read_exact(&mut buf).unwrap();
        core::str::from_utf8(&buf).unwrap().to_owned()
    });
    println!(
        "\tCreator Revision: {:#X?}",
        cursor.read_u32::<LittleEndian>().unwrap()
    );
    println!("\tTable UUID: {:X?}", {
        let mut buf = [0; 16];
        cursor.read_exact(&mut buf).unwrap();
        buf
    });
    let vbios_image_offset = cursor.read_u32::<LittleEndian>().unwrap();
    println!("\tVBIOS Image Offset: {:#X?}", vbios_image_offset);
    let lib1_image_offset = cursor.read_u32::<LittleEndian>().unwrap();
    println!("\tLib1 Image Offset: {:#X?}", lib1_image_offset);
    assert_ne!(vbios_image_offset, 0);
    cursor.set_position(vbios_image_offset.into());
    println!("\nVBIOS Image:");
    println!(
        "\tPCI Bus: {:#X?}",
        cursor.read_u32::<LittleEndian>().unwrap()
    );
    println!(
        "\tPCI Device: {:#X?}",
        cursor.read_u32::<LittleEndian>().unwrap()
    );
    println!(
        "\tPCI Function: {:#X?}",
        cursor.read_u32::<LittleEndian>().unwrap()
    );
    println!(
        "\tVendor ID: {:#X?}",
        cursor.read_u16::<LittleEndian>().unwrap()
    );
    println!(
        "\tDevice ID: {:#X?}",
        cursor.read_u16::<LittleEndian>().unwrap()
    );
    println!(
        "\tSSVID: {:#X?}",
        cursor.read_u16::<LittleEndian>().unwrap()
    );
    println!("\tSSID: {:#X?}", cursor.read_u16::<LittleEndian>().unwrap());
    println!(
        "\tRevision: {:#X?}",
        cursor.read_u32::<LittleEndian>().unwrap()
    );
    let image_length = cursor.read_u32::<LittleEndian>().unwrap();
    println!("\tImage Length: {:#X?}", image_length);

    let mut buf = Vec::new();
    buf.resize(image_length as usize, 0);
    cursor.read_exact(buf.as_mut_slice()).unwrap();
    if image_length < 64 * 1000 {
        buf.resize(64 * 1000, 0);
    }
    println!("Saving extracted VBIOS to {}", output);
    std::fs::write(output, buf).unwrap();
}
