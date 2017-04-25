extern crate itertools;
extern crate unicode_segmentation;

use std::io::BufReader;
use std::{io};

use itertools::Itertools;
use std::io::BufRead;
use unicode_segmentation::UnicodeSegmentation;

const LOOKUP_TABLE : [char; 256] =
[
    '⠀', '⠁', '⠂', '⠃', '⠄', '⠅', '⠆', '⠇', '⡀', '⡁', '⡂', '⡃', '⡄', '⡅', '⡆', '⡇',
'⠈', '⠉', '⠊', '⠋', '⠌', '⠍', '⠎', '⠏', '⡈', '⡉', '⡊', '⡋', '⡌', '⡍', '⡎', '⡏',
'⠐', '⠑', '⠒', '⠓', '⠔', '⠕', '⠖', '⠗', '⡐', '⡑', '⡒', '⡓', '⡔', '⡕', '⡖', '⡗',
'⠘', '⠙', '⠚', '⠛', '⠜', '⠝', '⠞', '⠟', '⡘', '⡙', '⡚', '⡛', '⡜', '⡝', '⡞', '⡟',
'⠠', '⠡', '⠢', '⠣', '⠤', '⠥', '⠦', '⠧', '⡠', '⡡', '⡢', '⡣', '⡤', '⡥', '⡦', '⡧',
'⠨', '⠩', '⠪', '⠫', '⠬', '⠭', '⠮', '⠯', '⡨', '⡩', '⡪', '⡫', '⡬', '⡭', '⡮', '⡯',
'⠰', '⠱', '⠲', '⠳', '⠴', '⠵', '⠶', '⠷', '⡰', '⡱', '⡲', '⡳', '⡴', '⡵', '⡶', '⡷',
'⠸', '⠹', '⠺', '⠻', '⠼', '⠽', '⠾', '⠿', '⡸', '⡹', '⡺', '⡻', '⡼', '⡽', '⡾', '⡿',
'⢀', '⢁', '⢂', '⢃', '⢄', '⢅', '⢆', '⢇', '⣀', '⣁', '⣂', '⣃', '⣄', '⣅', '⣆', '⣇',
'⢈', '⢉', '⢊', '⢋', '⢌', '⢍', '⢎', '⢏', '⣈', '⣉', '⣊', '⣋', '⣌', '⣍', '⣎', '⣏',
'⢐', '⢑', '⢒', '⢓', '⢔', '⢕', '⢖', '⢗', '⣐', '⣑', '⣒', '⣓', '⣔', '⣕', '⣖', '⣗',
'⢘', '⢙', '⢚', '⢛', '⢜', '⢝', '⢞', '⢟', '⣘', '⣙', '⣚', '⣛', '⣜', '⣝', '⣞', '⣟',
'⢠', '⢡', '⢢', '⢣', '⢤', '⢥', '⢦', '⢧', '⣠', '⣡', '⣢', '⣣', '⣤', '⣥', '⣦', '⣧',
'⢨', '⢩', '⢪', '⢫', '⢬', '⢭', '⢮', '⢯', '⣨', '⣩', '⣪', '⣫', '⣬', '⣭', '⣮', '⣯',
'⢰', '⢱', '⢲', '⢳', '⢴', '⢵', '⢶', '⢷', '⣰', '⣱', '⣲', '⣳', '⣴', '⣵', '⣶', '⣷',
'⢸', '⢹', '⢺', '⢻', '⢼', '⢽', '⢾', '⢿', '⣸', '⣹', '⣺', '⣻', '⣼', '⣽', '⣾', '⣿',
];

/// Convert simple binary representation that this crate
/// uses to unicode character, by calculating the right unicode
///
/// Unicode offset format
///
/// ```norust
/// 03
/// 14
/// 25
/// 67
/// ```
///
/// This crate format:
/// ```norust
/// 04
/// 15
/// 26
/// 37
/// ```
fn byte_to_braillechar(nb : u8) -> char {

    let nb = nb as u32;
    let mut ub = 0u32;
    ub |= ((nb >> 0) & 0x7) << 0;
    ub |= ((nb >> 4) & 0x7) << 3;
    ub |= ((nb >> 3) & 0x1) << 6;
    ub |= ((nb >> 7) & 0x1) << 7;



    std::char::from_u32(0x2800u32 + ub).unwrap()
}

#[allow(unused)]
/// This is the script that generated `LOOKUP_TABLE`
fn generate_lookup_table() {
    for bin2 in 0..16 {
        println!(
            "'{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}',",
            byte_to_braillechar(bin2 * 16 + 0),
            byte_to_braillechar(bin2 * 16 + 1),
            byte_to_braillechar(bin2 * 16 + 2),
            byte_to_braillechar(bin2 * 16 + 3),
            byte_to_braillechar(bin2 * 16 + 4),
            byte_to_braillechar(bin2 * 16 + 5),
            byte_to_braillechar(bin2 * 16 + 6),
            byte_to_braillechar(bin2 * 16 + 7),
            byte_to_braillechar(bin2 * 16 + 8),
            byte_to_braillechar(bin2 * 16 + 9),
            byte_to_braillechar(bin2 * 16 + 10),
            byte_to_braillechar(bin2 * 16 + 11),
            byte_to_braillechar(bin2 * 16 + 12),
            byte_to_braillechar(bin2 * 16 + 13),
            byte_to_braillechar(bin2 * 16 + 14),
            byte_to_braillechar(bin2 * 16 + 15),
            )
    }
}

const BRAILLE_Y :usize = 4;
#[allow(unused)]
const BRAILLE_X :usize = 2;

/// Generate minimap/preview for the text read from `reader`j
pub fn to_minimap<'a, R>(reader: R, settings: Settings) -> Box<Iterator<Item = String> + 'a>
    where R: io::Read + 'a
{

    let i = to_minimap_bool(reader, settings)
        .batching(move |i| {
            let mut group : Vec<Vec<bool>> = Vec::new();
            for _ in 0..BRAILLE_Y {
                if let Some(line) = i.next() {
                    group.push(line);
                } else {
                    break;
                }
            }
            if group.is_empty() {
                None
            } else {
                Some(group)
            }
        })
        // map to binary representations of 4 bits in every column
        .map(move |group| {
            let mut line_iters : Vec<_> = group
                .iter()
                .map(|line| line.iter()).collect();

            let mut binary_column : Vec<u8> = Vec::new();


            loop {
                // reset to false if anything was found
                let mut finished = true;

                let mut binary = 0;

                for (row_i, mut line_iter) in line_iters.iter_mut().enumerate() {
                    if let Some(&printable) = line_iter.next() {
                        finished = false;
                        if printable {
                            binary |= 1 << row_i;
                        }
                    }
                }

                if finished {
                    break;
                }

                binary_column.push(binary)
            }

            binary_column
        })
        .map(|line| {
            let mut s = String::new();

            let mut iter = line.iter();
            loop {
                let bin1 = iter.next();
                let bin2 = iter.next();

                if bin1.is_none() && bin2.is_none() {
                    break;
                }

                let bin1 : u8 = *bin1.unwrap_or(&0);
                let bin2 : u8 = *bin2.unwrap_or(&0);

                debug_assert!(bin1 < 16);
                debug_assert!(bin2 < 16);

                let ch = LOOKUP_TABLE[(bin2 * 16 + bin1) as usize];
                debug_assert_eq!(ch, byte_to_braillechar((bin2 * 16 + bin1)));
                s.push(ch)
            }

            s
        });

    Box::new(i)
}

#[allow(unused)]
fn to_minimap_stars<'a, R>(reader: R, settings: Settings) -> Box<Iterator<Item = String> + 'a>
    where R: io::Read + 'a
{

    let i = to_minimap_bool(reader, settings)
        .map(|dots| {
            let v : Vec<_> = dots.iter().map(|&b| if b { "*" } else { " " }).collect();
            let line : String = v.join("");
            line
        });

    Box::new(i)
}

fn to_minimap_bool<'a, R>(reader: R,
                          settings: Settings)
                          -> Box<Iterator<Item = Vec<bool>> + 'a>
    where R: io::Read + 'a
{

    let buf_reader = BufReader::new(reader);

    let xscale = settings.xscale;
    let yscale = settings.yscale;

    let i = buf_reader
        .lines()
        .take_while(|b| b.is_ok())
        .map(|b| b.unwrap())
        // iterate over groups of n lines
        .batching(move |i| {
            let mut group : Vec<String> = Vec::new();
            for _ in 0..yscale {
                if let Some(line) = i.next() {
                    group.push(line);
                } else {
                    break;
                }
            }
            if group.is_empty() {
                None
            } else {
                Some(group)
            }
        })
        .map(move |group| {
            let mut line_iters : Vec<_> = group
                .iter()
                .map(|line| UnicodeSegmentation::graphemes(line.as_str(), true)).collect();

            let mut dots : Vec<bool> = Vec::new();
            loop {
                let mut in_this_column : Vec<bool> = Vec::new();
                for line_iter in &mut line_iters {
                    let mut contains_nonwhite = None;

                    for _ in 0..xscale {
                        if let Some(glyph) = line_iter.next() {
                            if contains_nonwhite != Some(true) {
                                if glyph.chars().any(|ch| !ch.is_whitespace()) {
                                    contains_nonwhite = Some(true);
                                } else {
                                    contains_nonwhite = Some(false);
                                }
                            }
                        }
                    }

                    if let Some(nw) = contains_nonwhite {
                        in_this_column.push(nw)
                    }
                }

                if in_this_column.is_empty() {
                    break;
                }

                dots.push(in_this_column.iter().any(|&g| g ))
            }
            dots
        });

    Box::new(i)
}

#[derive(Clone)]
pub struct Settings {
    pub xscale: usize,
    pub yscale: usize,
}

impl Settings {
    pub fn new() -> Self {
        Settings {
            xscale: 1,
            yscale: 1,
        }
    }
}

