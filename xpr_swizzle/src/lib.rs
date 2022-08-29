// Copyright (c) 2022 Darren Thompson
//
// Code ported from img-to-xpr.py
//
// Swizzle code derived from swizzle.c
//
//  Copyright (c) 2022 Matt Borgerson
//  Copyright (c) 2015 Jannik Vogel
//  Copyright (c) 2013 espes
//  Copyright (c) 2007-2010 The Nouveau Project.
//
//  This library is free software; you can redistribute it and/or
//  modify it under the terms of the GNU Lesser General Public
//  License as published by the Free Software Foundation; either
//  version 2 of the License, or (at your option) any later version.
//
//  This library is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
//  Lesser General Public License for more details.
//
//  You should have received a copy of the GNU Lesser General Public
//  License along with this library; if not, see <http://www.gnu.org/licenses/>.

use std::mem;
pub use packed_struct::prelude::*;

struct SwizzleMask {
    x: u32,
    y: u32,
    z: u32,
}

#[derive(PackedStruct)]
#[packed_struct(endian="lsb")]
pub struct XPRHeader {
    // XPR Header
    pub magic: u32,
    pub total_size: u32,
    pub header_size: u32,
    // D3D Texture
    pub common: u32,
    pub data: u32,
    pub lock: u32,
    pub format: u32,
    pub size: u32,
    pub end_of_header: u32,
}

fn generate_swizzle_masks(width: u32, height: u32, depth: u32) -> SwizzleMask {
    let mut x: u32 = 0;
    let mut y: u32 = 0;
    let mut z: u32 = 0;

    let mut bit: u32 = 1;
    let mut mask_bit: u32 = 1;

    let mut done = false;

    while !done {
        done = true;

        if bit < width {
            x |= mask_bit;
            mask_bit <<= 1;
            done = false;
        }

        if bit < height {
            y |= mask_bit;
            mask_bit <<= 1;
            done = false;
        }

        if bit < depth {
            z |= mask_bit;
            mask_bit <<= 1;
            done = false;
        }

        bit <<= 1;
    }

    assert_eq!(x ^ y ^ z, (mask_bit - 1));

    return SwizzleMask { x, y, z };
}

fn fill_pattern(pattern: u32, value: u32) -> u32 {
    let mut result: u32 = 0;
    let mut bit: u32 = 1;
    let mut local_value: u32 = value;

    while local_value != 0 {
        if (pattern & bit) != 0 {
            let mut tmp: u32 = 0;
            if (local_value & 1) != 0 {
                tmp = bit;
            }

            result |= tmp;
            local_value >>= 1;
        }

        bit <<= 1;
    }

    return result;
}

fn get_swizzled_offset(x: u32, y: u32, z: u32, mask: &SwizzleMask) -> usize {
    let result = (fill_pattern(mask.x, x) | fill_pattern(mask.y, y) | fill_pattern(mask.z, z)) * 4;
    return result as usize;
}

pub fn swizzle_box(src_buf: &mut Vec<u8>, width: u32, height: u32, depth: u32, dst_buf: &mut Vec<u8>, row_pitch: u32) {
    let mask = generate_swizzle_masks(width, height, depth);

    for z in 0..depth {
        for y in 0..height {
            for x in 0..width {
                let left = get_swizzled_offset(x, y, z, &mask);
                let right = ((y * row_pitch + x) * 4) as usize;

                // Convert to BGRA
                // R
                dst_buf[left + 2] = src_buf[right];
                // G
                dst_buf[left + 1] = src_buf[right + 1];
                // B
                dst_buf[left] = src_buf[right + 2];
                // A
                dst_buf[left + 3] = src_buf[right + 3];
            }
        }
    }
}

pub fn create_header(img_buf: &Vec<u8>) -> Result<[u8; 36], PackingError> {
    let header_size = mem::size_of::<XPRHeader>() as u32;
    let header = XPRHeader {
        magic: 0x30525058,
        header_size,
        total_size: header_size + (img_buf.len() as u32),
        common: 0x40001,
        data: 0,
        lock: 0,
        format: 0x6610629,
        size: 0,
        end_of_header: 0xffffffff
    };

    return header.pack();
}
