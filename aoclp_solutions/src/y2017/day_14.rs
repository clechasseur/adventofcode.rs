use std::collections::HashSet;

use aoclp::positioning::pt::Pt;
use aoclp::solvers_impl::input::safe_get_input;

use crate::y2017::helpers::knot_hash::KnotHash;

pub fn part_1() -> u32 {
    Disk::sda().used_count()
}

pub fn part_2() -> usize {
    Disk::sda().regions_count()
}

struct Disk {
    hashes: Vec<KnotHash>,
}

impl Disk {
    const NEIGHBOURS: [Pt; 4] = [Pt::new(-1, 0), Pt::new(0, -1), Pt::new(1, 0), Pt::new(0, 1)];

    pub fn sda() -> Self {
        let input = input();
        Self {
            hashes: (0..128)
                .map(|row| KnotHash::new(format!("{input}-{row}")))
                .collect(),
        }
    }

    pub fn used_count(&self) -> u32 {
        self.hashes
            .iter()
            .map(|h| h.dense().iter().map(|n| n.count_ones()).sum::<u32>())
            .sum()
    }

    pub fn regions_count(&self) -> usize {
        let mut seen = HashSet::new();
        let mut count = 0;

        for y in 0..128 {
            for x in 0..128 {
                let pt = Pt::new(x, y);

                if !seen.contains(&pt) {
                    let mut region = HashSet::new();
                    self.fill_region(pt, &mut region);

                    if !region.is_empty() {
                        count += 1;
                        seen.extend(region);
                    }
                }
            }
        }

        count
    }

    fn used_at(&self, pt: Pt) -> bool {
        if pt.x < 0 || pt.y < 0 {
            return false;
        }

        self.hashes
            .get(pt.y as usize)
            .and_then(|h| {
                h.dense()
                    .get((pt.x / 8) as usize)
                    .map(|&n| n & (1 << (7 - pt.x % 8)) != 0)
            })
            .unwrap_or(false)
    }

    fn fill_region(&self, pt: Pt, region: &mut HashSet<Pt>) {
        if !region.contains(&pt) && self.used_at(pt) {
            region.insert(pt);

            Self::NEIGHBOURS
                .iter()
                .map(|&n| pt + n)
                .for_each(|n| self.fill_region(n, region));
        }
    }
}

fn input() -> String {
    safe_get_input(2017, 14)
}
