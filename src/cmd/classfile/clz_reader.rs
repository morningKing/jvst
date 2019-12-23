extern crate hex;
use std::convert::TryInto;
use std::mem;

pub fn read_u8_string(data: &[u8], mut res: String, mut index: u32) -> (String, u32) {
    let mut tmparr: [u8; 1] = [0; 1];
    let end = index + 1;
    let mut tix = 0;
    for (i, n) in data.iter().enumerate() {
        if i >= index.try_into().unwrap() && i < end.try_into().unwrap() {
            tmparr[tix] = *n;
            tix = tix + 1;
        } else if i == end.try_into().unwrap() {
            break;
        }
    }
    index = index + 1;
    res = hex::encode(tmparr);
    (res, index)
}

pub fn read_u8(data: &[u8], mut res: u8, mut index: u32) -> (u8, u32) {
    let mut tmparr: [u8; 1] = [0; 1];
    let end = index + 1;
    let mut tix = 0;
    for (i, n) in data.iter().enumerate() {
        if i >= index.try_into().unwrap() && i < end.try_into().unwrap() {
            tmparr[tix] = *n;
            tix = tix + 1;
        } else if i == end.try_into().unwrap() {
            break;
        }
    }
    index = index + 1;
    res = tmparr[0];
    (res, index)
}

pub fn read_u16_string(data: &[u8], mut res: String, mut index: u32) -> (String, u32) {
    let mut tmparr: [u8; 2] = [0; 2];
    let end = index + 2;
    let mut tix = 0;
    for (i, n) in data.iter().enumerate() {
        if i >= index.try_into().unwrap() && i < end.try_into().unwrap() {
            tmparr[tix] = *n;
            tix = tix + 1;
        } else if i == end.try_into().unwrap() {
            break;
        }
    }
    index = index + 2;
    res = hex::encode(tmparr);
    (res, index)
}

pub fn read_u16(data: &[u8], mut res: u16, mut index: u32) -> (u16, u32) {
    let mut tmparr: [u8; 2] = [0; 2];
    let end = index + 2;
    let mut tix = 0;
    for (i, n) in data.iter().enumerate() {
        if i >= index.try_into().unwrap() && i < end.try_into().unwrap() {
            tmparr[tix] = *n;
            tix = tix + 1;
        } else if i == end.try_into().unwrap() {
            break;
        }
    }
    index = index + 2;
    unsafe {
        tmparr.reverse(); //transmute函数要对传入的数组做逆序处理 如[0,1] => 256 [1,0] => 1
        res = mem::transmute::<[u8; 2], u16>(tmparr);
    }
    (res, index)
}

pub fn read_u32_string(data: &[u8], mut res: String, mut index: u32) -> (String, u32) {
    let mut tmparr: [u8; 4] = [0; 4];
    let end = index + 4;
    let mut tix = 0;
    for (i, n) in data.iter().enumerate() {
        if i >= index.try_into().unwrap() && i < end.try_into().unwrap() {
            tmparr[tix] = *n;
            tix = tix + 1;
        } else if i == end.try_into().unwrap() {
            break;
        }
    }
    index = index + 4;
    res = hex::encode(tmparr);
    (res, index)
}
