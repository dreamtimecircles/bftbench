use std::fmt::Display;

use histogram::Histogram;
use serde_derive::Serialize;

#[derive(Debug, Clone)]
pub(crate) struct WriteHistograms {
    pub(crate) successful_nanos: Histogram,
    pub(crate) failed_nanos: Histogram,
}

impl WriteHistograms {
    pub(crate) fn new() -> Self {
        WriteHistograms {
            successful_nanos: Histogram::new(),
            failed_nanos: Histogram::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ReadHistograms {
    pub(crate) successful_nanos: Histogram,
    pub(crate) failed_nanos: Histogram,
    pub(crate) roundtrip_nanos: Histogram,
}

impl ReadHistograms {
    pub(crate) fn new() -> Self {
        ReadHistograms {
            successful_nanos: Histogram::new(),
            failed_nanos: Histogram::new(),
            roundtrip_nanos: Histogram::new(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Histograms {
    pub(crate) global_write_histograms: WriteHistograms,
    pub(crate) global_read_histograms: ReadHistograms,
    pub(crate) nodes_write_histograms: Vec<WriteHistograms>,
    pub(crate) nodes_read_histograms: Vec<ReadHistograms>,
}

impl Histograms {
    pub(crate) fn new(nodes_count: usize, read_nodes_count: usize) -> Self {
        Histograms {
            global_write_histograms: WriteHistograms::new(),
            global_read_histograms: ReadHistograms::new(),
            nodes_write_histograms: vec![WriteHistograms::new(); nodes_count],
            nodes_read_histograms: vec![ReadHistograms::new(); read_nodes_count],
        }
    }
}

#[derive(Serialize)]
pub struct WriteStats {
    writes_successful_nanos_avg: Result<u64, &'static str>,
    writes_failed_nanos_avg: Result<u64, &'static str>,
}

#[derive(Serialize)]
pub struct ReadStats {
    read_successful_nanos_avg: Result<u64, &'static str>,
    read_failed_nanos_avg: Result<u64, &'static str>,
    roundtrip_nanos_avg: Result<u64, &'static str>,
}

#[derive(Serialize)]
pub struct Stats {
    global_write_stats: WriteStats,
    global_read_stats: ReadStats,
    nodes_write_stats: Vec<WriteStats>,
    nodes_read_stats: Vec<ReadStats>,
}

impl From<Histograms> for Stats {
    fn from(histograms: Histograms) -> Self {
        let mut stats = Stats {
            global_write_stats: WriteStats {
                writes_successful_nanos_avg: histograms
                    .global_write_histograms
                    .successful_nanos
                    .mean(),
                writes_failed_nanos_avg: histograms
                    .global_write_histograms
                    .failed_nanos
                    .mean(),
            },
            global_read_stats: ReadStats {
                read_successful_nanos_avg: histograms
                    .global_read_histograms
                    .successful_nanos
                    .mean(),
                read_failed_nanos_avg: histograms
                    .global_read_histograms
                    .failed_nanos
                    .mean(),
                roundtrip_nanos_avg: histograms
                    .global_read_histograms
                    .roundtrip_nanos
                    .mean(),
            },
            nodes_write_stats: vec![],
            nodes_read_stats: vec![],
        };
        histograms.nodes_write_histograms.into_iter().for_each(|write_histograms| {
            let write_stats = WriteStats {
                writes_successful_nanos_avg: write_histograms.successful_nanos.mean(),
                writes_failed_nanos_avg: write_histograms.failed_nanos.mean(),
            };
            stats.nodes_write_stats.push(write_stats);
        });
        histograms.nodes_read_histograms.into_iter().for_each(|read_histograms| {
            let read_stats = ReadStats {
                read_successful_nanos_avg: read_histograms.successful_nanos.mean(),
                read_failed_nanos_avg: read_histograms.failed_nanos.mean(),
                roundtrip_nanos_avg: read_histograms.roundtrip_nanos.mean(),
            };
            stats.nodes_read_stats.push(read_stats);
        });
        stats
    }
}

impl Display for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let json = serde_json::to_string(self).map_err(|_| std::fmt::Error)?;
        write!(f, "{}", json)
    }
}
