use std::fmt::Display;

use histogram::Histogram;

#[derive(Debug)]
pub struct Stats {
    pub(crate) global_writes_successful_nanos: Histogram,
    pub(crate) global_writes_failed_nanos: Histogram,
    pub(crate) node_writes_successful_nanos: Vec<Histogram>,
    pub(crate) node_writes_failed_nanos: Vec<Histogram>,
    pub(crate) global_reads_successful_nanos: Histogram,
    pub(crate) global_reads_failed_nanos: Histogram,
    pub(crate) node_reads_successful_nanos: Vec<Histogram>,
    pub(crate) node_reads_failed_nanos: Vec<Histogram>,
    pub(crate) global_roundtrip_nanos: Histogram,
    pub(crate) nodes_roundtrip_nanos: Vec<Histogram>,
}

impl Stats {
    pub(crate) fn new(nodes_count: usize, read_nodes_count: usize) -> Self {
        Stats {
            global_writes_successful_nanos: Histogram::new(),
            global_writes_failed_nanos: Histogram::new(),
            node_writes_successful_nanos: vec![Histogram::new(); nodes_count],
            node_writes_failed_nanos: vec![Histogram::new(); nodes_count],
            global_reads_successful_nanos: Histogram::new(),
            global_reads_failed_nanos: Histogram::new(),
            node_reads_successful_nanos: vec![Histogram::new(); read_nodes_count],
            node_reads_failed_nanos: vec![Histogram::new(); read_nodes_count],
            global_roundtrip_nanos: Histogram::new(),
            nodes_roundtrip_nanos: vec![Histogram::new(); read_nodes_count],
        }
    }
}

impl Display for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        indoc::writedoc!(
            f,
            "
            {{
                global-writes: {{
                    successful: {{
                        count: {}
                        avg-duration: {}
                    }}
                    failed: {{
                        count: {}
                    }}
                }}
                global-reads: {{
                    successful: {{
                        count: {}
                        avg-duration: {}
                    }}
                    failed: {{
                        count: {}
                    }}
                }}
                global-roundtrips: {{
                    count: {}
                    avg-duration: {}
                }}
            }}",
            self.global_writes_successful_nanos.entries(),
            self.global_writes_successful_nanos.mean().unwrap(),
            self.global_writes_failed_nanos.entries(),
            self.global_reads_successful_nanos.entries(),
            self.global_reads_successful_nanos.mean().unwrap(),
            self.global_reads_failed_nanos.entries(),
            self.global_roundtrip_nanos.entries(),
            self.global_roundtrip_nanos.mean().unwrap(),
        )
    }
}
