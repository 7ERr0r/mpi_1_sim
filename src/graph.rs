use rand::prelude::*;
use rand_pcg::Pcg64;

#[derive(Clone, Default)]
pub struct QueueProbabilities {
    //
    // prob[i] = prawdopodobienstwo trafienia do kolejki i
    prob: Vec<f64>,

    // suma wszystkich prawodpodobienstw
    sum: f64,
}

#[derive(Clone)]
pub struct MpiGraph {
    pub num_nodes: usize,
    pub queue_outputs: Vec<QueueProbabilities>,
}

impl MpiGraph {
    pub fn new(num_nodes: usize) -> Self {
        // Domyslnie prawdopodobienstwo dla kazdego
        // wierzcholka: A -> B jest rowne 0

        let mut queue_outputs = Vec::new();
        for _ in 0..=num_nodes {
            let mut probs = Vec::new();
            for _ in 0..=num_nodes {
                probs.push(0.0);
            }
            queue_outputs.push(QueueProbabilities {
                prob: probs,
                sum: 0.0,
            });
        }

        Self {
            queue_outputs,
            num_nodes,
        }
    }
    pub fn new_from_random_seed(num_nodes: usize, seed: u64) -> Self {
        let mut rng = Pcg64::seed_from_u64(seed);

        let mut graph = Self::new(num_nodes);

        for a in 0..=num_nodes {
            for b in 0..=num_nodes {
                let prob = rng.gen_range(0.0001..1.0);
                graph.connect(a, b, prob);
            }
        }
        // wejscie nie moze od razu wejsc
        // na wyjscie z sieci (pstwo 0.0)
        graph.connect(num_nodes, num_nodes, 0.0);

        graph.sum_probabilities();

        graph
    }

    pub fn connect(&mut self, id_a: usize, mut id_b: usize, probability: f64) {
        if id_b > self.queue_outputs.len() {
            // nieistniejaca kolejka
            // wyjscie z sieci jacksona
            id_b = self.queue_outputs.len();
        }
        self.queue_outputs[id_a].prob[id_b] = probability;
    }
    pub fn sum_probabilities(&mut self) {
        for qprobabilities in &mut self.queue_outputs {
            qprobabilities.sum = qprobabilities.prob.iter().sum();
        }
    }

    pub fn fix_cycle_prevent_self(&mut self) {
        for (i, qprobabilities) in self.queue_outputs.iter_mut().enumerate() {
            // i-ta kolejka nie moze
            // wyslac pakietu do siebie
            qprobabilities.prob[i] = 0.0;
        }
    }
    pub fn fix_cycle_prevent_lower_id(&mut self) {
        for i in 0..self.num_nodes {
            let q = &mut self.queue_outputs[i];
            // i-ta kolejka nie moze wyslac pakietu do
            // innej o mniejszym id
            for j in 0..=i {
                q.prob[j] = 0.0;
            }
        }
    }

    pub fn randomize_next_node(&self, rng: &mut Pcg64, current_node_id: usize) -> usize {
        let probabilities = &self.queue_outputs[current_node_id];

        let sum = probabilities.sum;
        if sum <= 0.0 {
            panic!("randomize_next_node: suma prawdopodobienstw jest rowna 0");
        }
        let r = rng.gen_range(0.0..sum);
        //println!("los {:6.3} na {:6.3}", r, sum);
        let mut current_sum = 0.0;
        for (i, &prob) in probabilities.prob.iter().enumerate() {
            current_sum += prob;
            //println!("current_sum {:6.3}", current_sum);
            if r < current_sum {
                //println!("wybrano {}", i);
                return i;
            }
        }
        return self.num_nodes;
    }

    pub fn probablity_table(&self) -> Vec<Vec<f64>> {
        let prob = &self.queue_outputs;

        let mut table = Vec::new();

        for p in prob {
            table.push(p.prob.clone());
        }

        table
    }
}
