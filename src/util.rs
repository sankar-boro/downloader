#[allow(unused)]

use regex::Regex;
use lazy_static::lazy_static;
use rand::Rng;
use std::cmp::{min, max};
use std::primitive::i16;

lazy_static! {
    static ref IPV6_REGEX: Regex = Regex::new(r"^(([0-9a-f]{1,4}:)(:[0-9a-f]{1,4}){1,6}|([0-9a-f]{1,4}:){1,2}(:[0-9a-f]{1,4}){1,5}|([0-9a-f]{1,4}:){1,3}(:[0-9a-f]{1,4}){1,4}|([0-9a-f]{1,4}:){1,4}(:[0-9a-f]{1,4}){1,3}|([0-9a-f]{1,4}:){1,5}(:[0-9a-f]{1,4}){1,2}|([0-9a-f]{1,4}:){1,6}(:[0-9a-f]{1,4})|([0-9a-f]{1,4}:){1,7}(([0-9a-f]{1,4})|:))\/(1[0-1]\d|12[0-8]|\d{1,2})$").unwrap();
    // static ref ESCAPING_SEQUENZES = [
    //   // Strings
    //   { start: '"', end: '"' },
    //   { start: "'", end: "'" },
    //   { start: '`', end: '`' },
    //   // RegeEx
    //   { start: '/', end: '/', startPrefix: /(^|[[{:;,/])\s?$/ },
    //   { x: Regex::new(r"^(([0-9a-f]{1,4}:)(:[0-9a-f]{1,4}){1,6}|([0-9a-f]{1,4}:){1,2}(:[0-9a-f]{1,4}){1,5}|([0-9a-f]{1,4}:){1,3}(:[0-9a-f]{1,4}){1,4}|([0-9a-f]{1,4}:){1,4}(:[0-9a-f]{1,4}){1,3}|([0-9a-f]{1,4}:){1,5}(:[0-9a-f]{1,4}){1,2}|([0-9a-f]{1,4}:){1,6}(:[0-9a-f]{1,4})|([0-9a-f]{1,4}:){1,7}(([0-9a-f]{1,4})|:))\/(1[0-1]\d|12[0-8]|\d{1,2})$").unwrap()}
    // ];
}

pub fn parseAbbreviatedNumber(str_: &str) -> Option<i32> {
    let match_ = str_
      .replace(",", ".")
      .replace(" ", "");
    //   /([\d,.]+)([MK]?)/
    //   .match();
    let is_match = Regex::new(r"^([\d,.]+)([MK]?)$").unwrap().is_match(&match_);
    if is_match {
    //   let [, num, multi] = match_;
    let num = 10;
    let multi = 'M';
    //   num = parseFloat(num);
        let num = match_.parse::<i32>().unwrap();
      if multi == 'M' {
        return Some(num * 1000000);
      } else {
        if multi == 'K' {
            return Some(num * 1000);
        } else {
            return Some(num);
        }
        // return Some(num); 
      }
    }
    return None;
}

fn format_radix(mut x: u32, radix: u32) -> String {
    let mut result = vec![];

    loop {
        let m = x % radix;
        x = x / radix;

        // will panic if you use a bad radix (< 2 or > 36).
        result.push(std::char::from_digit(m, radix).unwrap());
        if x == 0 {
            break;
        }
    }
    result.into_iter().rev().collect()
}

pub fn get_random_ipv6(ip: &str) -> Result<(), ()>   {
    let x = is_ipv6(ip);
    if !x {
        return Err(());   
    }
    let ips: Vec<&str> = ip.split('/').collect();
    let rawAddr = ips[0];
    let rawMask = ips[1];
    let mut     base10Mask: u16 = rawMask.parse().unwrap();

    if base10Mask != 0 || base10Mask > 128 || base10Mask < 24 {
        println!("true, {base10Mask}");
    }

    let base10addr = normalize_ip(rawAddr);
    println!("base10addr: {:?}", base10addr);
    let randomAddr: [u16; 8] = [1;8];
    let mut rng = rand::thread_rng();
    let randomAddr = randomAddr.map(|_| {
        rng.gen::<u16>() * 0xffff
    });

    let mergedAddr: Vec<u16> = randomAddr
    .iter()
    .enumerate()
    .map(|(idx, randomItem)| {
        // let idx = *idx as usize;
        let staticBits = min(base10Mask, 16);
        base10Mask -= staticBits;
        let mask = 0xffff - (((16 - staticBits).pow(2)) - 1);
        println!("mask {mask}");
        (base10addr[idx] & mask) + (randomItem & (mask ^ 0xffff))
    }).collect();

    let address_: Vec<String> = mergedAddr.iter().map(|x| {
        // let y = y.to_string_radix();
        format_radix(x.to_owned() as u32, 16)
    }).collect();

    // let y = 10;
    // let y: i32 = i32::from_str_radix(y, 16);
    println!("randomAddr: {:?}", randomAddr);
    println!("mergedAddr: {:?}", mergedAddr);
    // println!("address_: {:?}", address_);

    // let rand_ = rng.gen::<u32>() * 0xffff; // note: Math.floor not done
    // println!("{rand_}");

    Ok(())
}

pub fn is_ipv6(ip: &str) -> bool {
    IPV6_REGEX.is_match(ip)
}

pub fn normalize_ip(ip: &str) -> [u16; 8] {
  let parts: Vec<&str> = ip.split("::").collect();
  let parts: Vec<Vec<&str>> = parts.iter().map(|s| {
    let x: Vec<&str> = s.split(":").collect();
    x
  }).collect();

  let part_start = parts[0].clone();
  let mut part_end = Vec::new();

  if parts.len() == 2 {
    part_end = parts[1].clone();
    part_end.reverse();
  }

  let mut full_ip = [0;8];

  for i in 0..min(part_start.len(), 8) {
    full_ip[i] = u16::from_str_radix(part_start[i], 16).unwrap();
  }

  for i in 0..min(part_end.len(), 8) {
    full_ip[7 - i] = u16::from_str_radix(part_end[i], 16).unwrap();
  }

  full_ip
}


pub fn dec_to_hex(decimal_number: u32) -> String {
    let mut result = String::new();
    let hex_chars: Vec<char> = "0123456789ABCDEF".chars().collect();

    if decimal_number == 0 {
        return "0".to_string();
    }

    let mut num = decimal_number;

    while num > 0 {
        let remainder = num % 16;
        let hex_char = hex_chars[remainder as usize];
        result.insert(0, hex_char);
        num /= 16;
    }

    result
}
