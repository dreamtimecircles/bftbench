use std::ops::Sub;
use std::{fmt::Display, time::Instant};

use histogram::Histogram;
use serde_derive::Serialize;

#[derive(Clone)]
pub(crate) struct Counter {
    pub(crate) start: Instant,
    pub(crate) now: Instant,
    pub(crate) count: u64,
}

impl Counter {
    pub(crate) fn new(init: Instant) -> Self {
        Counter {
            start: init,
            now: init,
            count: 0,
        }
    }
}

#[derive(Clone)]
pub(crate) struct Stat {
    pub(crate) histogram: Histogram,
    pub(crate) counter: Counter,
}

impl Stat {
    pub(crate) fn new(init: Instant) -> Self {
        Stat {
            histogram: new_default_histogram(),
            counter: Counter::new(init),
        }
    }
}

#[derive(Clone)]
pub(crate) struct OpStat {
    pub(crate) successful: Stat,
    pub(crate) failed: Stat,
}

impl OpStat {
    pub(crate) fn new(init: Instant) -> Self {
        OpStat {
            successful: Stat::new(init),
            failed: Stat::new(init),
        }
    }
}

#[derive(Clone)]
pub(crate) struct ReadStat {
    pub(crate) op: OpStat,
    pub(crate) round_trip: Stat,
}

impl ReadStat {
    pub(crate) fn new(init: Instant) -> Self {
        ReadStat {
            op: OpStat::new(init),
            round_trip: Stat::new(init),
        }
    }
}

pub(crate) struct Stats {
    pub(crate) global_write: OpStat,
    pub(crate) global_read: ReadStat,
    pub(crate) nodes_writes: Vec<OpStat>,
    pub(crate) nodes_reads: Vec<ReadStat>,
}

impl Stats {
    pub(crate) fn new(init: Instant, write_nodes: usize, read_nodes: usize) -> Self {
        Stats {
            global_write: OpStat::new(init),
            global_read: ReadStat::new(init),
            nodes_writes: vec![OpStat::new(init); write_nodes],
            nodes_reads: vec![ReadStat::new(init); read_nodes],
        }
    }
}

#[derive(Serialize)]
pub struct Report {
    pub count: u64,
    pub rate_s: Option<u64>,
    pub micros_avg: Option<u64>,
    pub micros_95: Option<u64>,
    pub micros_99: Option<u64>,
}

impl From<&Stat> for Report {
    fn from(stat: &Stat) -> Self {
        Report {
            count: stat.counter.count,
            rate_s: if stat.counter.count == 0 {
                None
            } else {
                Some(
                    (1.0 / (stat
                        .counter
                        .now
                        .sub(stat.counter.start)
                        .div_f64(stat.counter.count as f64)
                        .as_secs_f64())) as u64,
                )
            },
            micros_avg: mean(&stat.histogram),
            micros_95: perc(95.0, &stat.histogram),
            micros_99: perc(99.0, &stat.histogram),
        }
    }
}

#[derive(Serialize)]
pub struct OpReport {
    pub successful: Report,
    pub failed: Report,
}

impl From<&OpStat> for OpReport {
    fn from(write_stat: &OpStat) -> Self {
        OpReport {
            successful: (&write_stat.successful).into(),
            failed: (&write_stat.failed).into(),
        }
    }
}

#[derive(Serialize)]
pub struct ReadReport {
    pub op: OpReport,
    pub round_trip: Report,
}

impl From<&ReadStat> for ReadReport {
    fn from(read_stat: &ReadStat) -> Self {
        ReadReport {
            op: (&read_stat.op).into(),
            round_trip: (&read_stat.round_trip).into(),
        }
    }
}

#[derive(Serialize)]
pub struct WithNodeIndex<R> {
    pub node_index: usize,
    pub report: R,
}

#[derive(Serialize)]
pub struct Reports {
    pub global_write: OpReport,
    pub global_read: ReadReport,
    pub nodes_writes: Vec<WithNodeIndex<OpReport>>,
    pub nodes_reads: Vec<WithNodeIndex<ReadReport>>,
}

impl From<&Stats> for Reports {
    fn from(stats: &Stats) -> Self {
        Reports {
            global_write: (&stats.global_write).into(),
            global_read: (&stats.global_read).into(),
            nodes_writes: into_reports(stats.nodes_writes.iter().collect()),
            nodes_reads: into_reports(stats.nodes_reads.iter().collect()),
        }
    }
}

fn into_reports<S, R: From<S>>(stats: Vec<S>) -> Vec<WithNodeIndex<R>> {
    stats
        .into_iter()
        .enumerate()
        .map(|(pos, x)| WithNodeIndex {
            node_index: pos,
            report: x.into(),
        })
        .collect()
}

impl Display for Reports {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let json = serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?;
        write!(f, "{}", json)
    }
}

fn new_default_histogram() -> Histogram {
    Histogram::new(16, 32).unwrap()
}

fn mean(histogram: &Histogram) -> Option<u64> {
    perc(50.0, histogram)
}

fn perc(percentile: f64, histogram: &Histogram) -> Option<u64> {
    histogram
        .percentile(percentile)
        .map(|bucket| bucket.end())
        .ok()
}
