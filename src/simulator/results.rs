use super::MpiParams;

#[derive(Default)]
pub struct MpiResults {
    num_nodes: usize,
    simulation_time: f64,

    // transmitted from a -> b
    // eg.
    // total_transmitted[a][b]
    pub total_transmitted: Vec<Vec<f64>>,

    // opoznienia w kolejce i
    // np.
    // delays[i] to sumaryczne opoznienia dla i
    pub delays: Vec<f64>,
}

impl MpiResults {
    pub fn new(params: &MpiParams) -> Self {
        let num_nodes = params.num_packet_queues;
        let mut results = MpiResults::default();
        results.num_nodes = num_nodes;
        results.delays = vec![0.0; num_nodes + 1];
        results.simulation_time = params.end_time;
        let mut aa = Vec::new();

        for _ in 0..=num_nodes {
            let mut bb = Vec::new();
            for _ in 0..=num_nodes {
                bb.push(0.0);
            }
            aa.push(bb);
        }

        results.total_transmitted = aa;

        results
    }

    pub fn add_transmitted(&mut self, a: usize, b: usize, transmitted: f64) {
        self.total_transmitted[a][b] += transmitted;
    }
    pub fn add_delays(&mut self, queue_id: usize, delay: f64) {
        self.delays[queue_id] += delay;
    }

    pub fn calc_throughputs(&self) -> Vec<Vec<f64>> {
        let mut aaa = Vec::new();
        for a in 0..=self.num_nodes {
            let mut bbb = Vec::new();
            for b in 0..=self.num_nodes {
                bbb.push(self.total_transmitted[a][b] / self.simulation_time);
            }
            aaa.push(bbb);
        }

        return aaa;
    }
}
