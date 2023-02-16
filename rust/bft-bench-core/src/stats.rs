use std::fmt::Display;

use histogram::Histogram;
use serde_derive::Serialize;

#[derive(Clone)]
pub(crate) struct WriteHistograms {
    pub(crate) successful_nanos: Histogram,
    pub(crate) failed_nanos: Histogram,
}

fn new_default_histogram() -> Histogram {
    Histogram::builder().build().unwrap()
}

impl WriteHistograms {
    pub(crate) fn new() -> Self {
        WriteHistograms {
            successful_nanos: new_default_histogram(),
            failed_nanos: new_default_histogram(),
        }
    }
}

#[derive(Clone)]
pub(crate) struct ReadHistograms {
    pub(crate) successful_nanos: Histogram,
    pub(crate) failed_nanos: Histogram,
    pub(crate) roundtrip_nanos: Histogram,
}

impl ReadHistograms {
    pub(crate) fn new() -> Self {
        ReadHistograms {
            successful_nanos: new_default_histogram(),
            failed_nanos: new_default_histogram(),
            roundtrip_nanos: new_default_histogram(),
        }
    }
}

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
    writes_successful_nanos_avg: Option<u64>,
    writes_failed_nanos_avg: Option<u64>,
}

#[derive(Serialize)]
pub struct ReadStats {
    read_successful_nanos_avg: Option<u64>,
    read_failed_nanos_avg: Option<u64>,
    roundtrip_nanos_avg: Option<u64>,
}

/// The stats currently produced.
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
                writes_successful_nanos_avg: mean(
                    histograms.global_write_histograms.successful_nanos,
                ),
                writes_failed_nanos_avg: mean(histograms.global_write_histograms.failed_nanos),
            },
            global_read_stats: ReadStats {
                read_successful_nanos_avg: mean(histograms.global_read_histograms.successful_nanos),
                read_failed_nanos_avg: mean(histograms.global_read_histograms.failed_nanos),
                roundtrip_nanos_avg: mean(histograms.global_read_histograms.roundtrip_nanos),
            },
            nodes_write_stats: vec![],
            nodes_read_stats: vec![],
        };
        histograms
            .nodes_write_histograms
            .into_iter()
            .for_each(|write_histograms| {
                let write_stats = WriteStats {
                    writes_successful_nanos_avg: mean(write_histograms.successful_nanos),
                    writes_failed_nanos_avg: mean(write_histograms.failed_nanos),
                };
                stats.nodes_write_stats.push(write_stats);
            });
        histograms
            .nodes_read_histograms
            .into_iter()
            .for_each(|read_histograms| {
                let read_stats = ReadStats {
                    read_successful_nanos_avg: mean(read_histograms.successful_nanos),
                    read_failed_nanos_avg: mean(read_histograms.failed_nanos),
                    roundtrip_nanos_avg: mean(read_histograms.roundtrip_nanos),
                };
                stats.nodes_read_stats.push(read_stats);
            });
        stats
    }
}

fn mean(histogram: Histogram) -> Option<u64> {
    histogram.percentile(50.0).map(|bucket| bucket.high()).ok()
}

impl Display for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let json = serde_json::to_string(self).map_err(|_| std::fmt::Error)?;
        write!(f, "{}", json)
    }
}
