use std::collections::HashMap;
use std::str::FromStr;
use std::sync::OnceLock;

use aoclp::anyhow::Context;
use aoclp::regex::Regex;
use aoclp::solvers_impl::input::safe_get_input_as_many;
use itertools::Itertools;

pub fn part_1() -> usize {
    let devices = devices_map();
    let mut cache = HashMap::new();
    num_paths(&devices, "you", true, true, &mut cache)
}

pub fn part_2() -> usize {
    let devices = devices_map();
    let mut cache = HashMap::new();
    num_paths(&devices, "svr", false, false, &mut cache)
}

fn devices_map() -> HashMap<String, Vec<String>> {
    input().into_iter().map(|d| (d.name, d.outputs)).collect()
}

fn num_paths(
    devices: &HashMap<String, Vec<String>>,
    cur: &str,
    dac: bool,
    fft: bool,
    cache: &mut HashMap<(String, bool, bool), usize>,
) -> usize {
    if let Some(num) = cache.get(&(cur.to_string(), dac, fft)) {
        return *num;
    }
    if cur == "out" {
        return if dac && fft { 1 } else { 0 };
    }

    let (dac, fft) = (dac || cur == "dac", fft || cur == "fft");
    let outputs = &devices[cur];
    let num = outputs
        .iter()
        .map(|d| num_paths(devices, d, dac, fft, cache))
        .sum();
    cache.insert((cur.to_string(), dac, fft), num);
    num
}

#[derive(Debug, Clone)]
struct Device {
    name: String,
    outputs: Vec<String>,
}

impl FromStr for Device {
    type Err = aoclp::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static REGEX: OnceLock<Regex> = OnceLock::new();
        let re =
            REGEX.get_or_init(|| Regex::new(r"^(?<name>\w+):\s+(?<outputs>(?:\w+\s*)+)$").unwrap());

        let captures = re
            .captures(s)
            .with_context(|| format!("invalid device spec: {s}"))?;
        let name = &captures["name"];
        let outputs = &captures["outputs"];
        let outputs = outputs.split_ascii_whitespace().map_into().collect_vec();

        Ok(Self { name: name.into(), outputs })
    }
}

fn input() -> Vec<Device> {
    safe_get_input_as_many(2025, 11)
}
