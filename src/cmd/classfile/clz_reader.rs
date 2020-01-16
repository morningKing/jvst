extern crate hex;
use std::convert::TryInto;
use std::mem;
use std::str;

fn pop64(barry: &[u8], start: u32, end: u32) -> &[u8; 8] {
    let start = start.try_into().unwrap();
    let end = end.try_into().unwrap();
    barry[start..end]
        .try_into()
        .expect("slice with incorrect length")
}

fn pop32(barry: &[u8], start: u32, end: u32) -> &[u8; 4] {
    let start = start.try_into().unwrap();
    let end = end.try_into().unwrap();
    barry[start..end]
        .try_into()
        .expect("slice with incorrect length")
}

fn pop16(barry: &[u8], start: u32, end: u32) -> &[u8; 2] {
    let start = start.try_into().unwrap();
    let end = end.try_into().unwrap();
    barry[start..end]
        .try_into()
        .expect("slice with incorrect length")
}

pub fn read_u8(data: &[u8], mut res: u8, mut index: &mut u32) -> u8 {
    let mut tmparr: [u8; 1] = [0; 1];
    let end = *index + 1;
    let mut tix = 0;
    for (i, n) in data.iter().enumerate() {
        if i >= (*index).try_into().unwrap() && i < end.try_into().unwrap() {
            tmparr[tix] = *n;
            tix = tix + 1;
        } else if i == end.try_into().unwrap() {
            break;
        }
    }
    *index = *index + 1;
    res = tmparr[0];
    res
}

pub fn read_u16_string(data: &[u8], mut res: String, index: &mut u32) -> (String) {
    let end = *index + 2;
    let tmparr = pop16(data, *index, end);
    *index = *index + 2;
    res = hex::encode(tmparr);
    res
}

pub fn read_u16(data: &[u8], mut index: &mut u32) -> u16 {
    let mut res = 0;
    let end = *index + 2;
    let tmparr = pop16(data, *index, end);
    *index = *index + 2;
    let mut arr = *tmparr;
    unsafe {
        arr.reverse(); //transmute函数要对传入的数组做逆序处理 如[0,1] => 256 [1,0] => 1
        res = mem::transmute::<[u8; 2], u16>(arr);
    }
    res
}

pub fn read_u32_string(data: &[u8], mut res: String, index: &mut u32) -> String {
    let end = *index + 4;
    let tmparr = pop32(data, *index, end);
    *index = *index + 4;
    res = hex::encode(tmparr);
    res
}

pub fn read_i32(data: &[u8], mut res: i32, index: &mut u32) -> i32 {
    let end = *index + 4;
    let tmparr = pop32(data, *index, end);
    *index = *index + 4;
    let mut arr = *tmparr;
    unsafe {
        arr.reverse(); //transmute函数要对传入的数组做逆序处理 如[0,1] => 256 [1,0] => 1
        res = mem::transmute::<[u8; 4], i32>(arr);
    }
    res
}

pub fn read_f32(data: &[u8], mut res: f32, index: &mut u32) -> f32 {
    let end = *index + 4;
    let tmparr = pop32(data, *index, end);
    *index = *index + 4;
    let mut arr = *tmparr;
    unsafe {
        arr.reverse(); //transmute函数要对传入的数组做逆序处理 如[0,1] => 256 [1,0] => 1
        res = mem::transmute::<[u8; 4], f32>(arr);
    }
    res
}

pub fn read_u32(data: &[u8], mut res: u32, index: &mut u32) -> u32 {
    let end = *index + 4;
    let tmparr = pop32(data, *index, end);
    *index = *index + 4;
    let mut arr = *tmparr;
    unsafe {
        arr.reverse(); //transmute函数要对传入的数组做逆序处理 如[0,1] => 256 [1,0] => 1
        res = mem::transmute::<[u8; 4], u32>(arr);
    }
    res
}

pub fn read_i64(data: &[u8], mut res: i64, index: &mut u32) -> i64 {
    let end = *index + 8;
    let tmparr = pop64(data, *index, end);
    *index = *index + 8;
    let mut arr = *tmparr;
    unsafe {
        arr.reverse(); //transmute函数要对传入的数组做逆序处理 如[0,1] => 256 [1,0] => 1
        res = mem::transmute::<[u8; 8], i64>(arr);
    }
    res
}

pub fn read_f64(data: &[u8], mut res: f64, mut index: &mut u32) -> f64 {
    let end = *index + 8;
    let tmparr = pop64(data, *index, end);
    *index = *index + 8;
    let mut arr = *tmparr;
    unsafe {
        arr.reverse(); //transmute函数要对传入的数组做逆序处理 如[0,1] => 256 [1,0] => 1
        res = mem::transmute::<[u8; 8], f64>(arr);
    }
    res
}

pub fn read_utf8(data: &[u8], mut res: String, index: &mut u32, len: u32) -> String {
    let start: usize = (*index).try_into().unwrap();
    let end: usize = (*index + len).try_into().unwrap();
    *index = *index + len;

    let temparr = &data[start..end];
    let uft8_str = str::from_utf8(temparr).unwrap();
    res = uft8_str.to_string();
    res
}

pub fn read_u16s(data: &[u8], index: &mut u32) -> Vec<u16> {
    let mut slice: Vec<u16> = Vec::new();
    let count = read_u16(data, index);
    for i in 0..count {
        let mut res = 0;
        res = read_u16(data, index);
        slice.push(res);
    }
    slice
}

pub fn read_u8s(data: &[u8], slice: &mut Vec<u8>, index: &mut u32, len: u32) {
    let start = *index;
    *index = *index + len;
    for i in start..*index {
        //try_into unwarp 转换成uszie
        let ix: usize = i.try_into().unwrap();
        slice.push(data[ix]);
    }
}
