pub mod analytics;
pub mod graph;
pub mod simulator;

//use analytics::*;
use std::fs::File;
use graph::*;
use simulator::*;
use std::io::Write;

fn main() {
    run(1500.0);
}

fn main_wykres() {
    let mut mi = 1050.0;
    let mut mi_results = vec![];
    for _i in 0..60 {
        mi += 50.0;
        let res = run(mi);
        mi_results.push((mi, res.0, res.1));
    }

    let mut file = File::create("wyniki.csv").unwrap();
    writeln!(file, "mi\tdelay_remember\tdelay_forget");
    for r in mi_results {
        writeln!(file, "{:.4}\t{:.4}\t{:.4}", r.0, r.1, r.2).unwrap();
    }
    

}


fn run(mi: f64) -> (f64, f64) {
    let num_packet_queues = 6;
    let hand_typed = true;
    let graph = if hand_typed {
        let outside_id = num_packet_queues;
        let mut graph = MpiGraph::new(num_packet_queues);
        graph.connect(0, 1, 1.0);
        graph.connect(1, 2, 1.0);
        graph.connect(2, 3, 1.0);
        graph.connect(3, 4, 1.0);
        graph.connect(4, 5, 1.0);
        //graph.connect(5, 0, 1.0);

        // prawdopodobienstwo:
        // wyjscia z sieci: 1.0 (z kolejki id 5)
        graph.connect(5, outside_id, 1.0);

        // prawdopodobienstwo:
        //      wejscia do sieci: zawsze rowne 1.0, w tym:
        // do wezla/kolejki id 0: 0.5
        // do wezla/kolejki id 1: 0.5
        graph.connect(outside_id, 0, 0.5);
        graph.connect(outside_id, 1, 0.5);

        graph.sum_probabilities();
        graph
    } else {
        let mut graph = MpiGraph::new_from_random_seed(num_packet_queues, 42);
        // proste usuwanie cykli
        graph.fix_cycle_prevent_self();
        graph.fix_cycle_prevent_lower_id();
        graph
    };

    let mut params = MpiParams {
        // lambda [bit/s]
        lambda: 1000.0,
        // mi     [bit/s]
        mi: mi,
        forget: false,
        num_packet_queues,
        ignore_time: 10.0,
        end_time: 500.0,
        seed: 42,
        verbose: false,
    };

    println!("\nsimulating non-forget");
    let mut sim_nf = MpiSimulator::new(params.clone(), graph.clone());
    sim_nf.simulate();
    println!("remaining pkts: {}", sim_nf.debug_remaining_packets());

    params.forget = true;

    println!("\nsimulating forget");
    let mut sim_f = MpiSimulator::new(params.clone(), graph.clone());
    sim_f.simulate();
    println!("remaining pkts: {}", sim_nf.debug_remaining_packets());

    // let mut analyzer = MpiAnalizator::new();
    // analyzer.analyze(graph);
    // println!("gamma_total_traffic: {:8.4}", analyzer.gamma_total_traffic);

    println!("\n\nprobabilities (routing):");
    print_table(&sim_nf.graph.probablity_table(), 1.0);

    println!("\n\nthroughputs from simulation (remember):");
    let throughputs_nf = sim_nf.results.calc_throughputs();
    print_table(&throughputs_nf, 1.0);

    println!("\n\nthroughputs from simulation (forget):");
    let throughputs_f = sim_f.results.calc_throughputs();
    print_table(&throughputs_f, 1.0);

    println!("\n\ndiff throughputs: (forget - remember)");

    let tp_diff = subtract_table(&throughputs_f, &throughputs_nf);
    print_table(&tp_diff, 10.0);

    let delays_nf = sim_nf.results.delays.clone();
    let delays_f = sim_f.results.delays.clone();

    let delays_diff: Vec<f64> = delays_f
        .iter()
        .zip(&delays_nf)
        .map(|(a, b)| a - b)
        .collect();

    let totalt = params.end_time;
    println!("\n\ndiff delays: (forget - remember)");
    for (i, qdelay) in delays_diff.iter().enumerate() {
        println!("{}: {:10.2}", i, qdelay / totalt);
    }

    let sum_r = delays_nf.iter().sum::<f64>() / totalt;
    let sum_f = delays_f.iter().sum::<f64>() / totalt;
    println!("sum delay remember: {:10.2}", sum_r);
    println!("sum delay   forget: {:10.2}", sum_f);

    return (sum_r, sum_f);
}

pub fn print_table(table: &Vec<Vec<f64>>, multiplier: f64) {
    print!("    ");
    for i in 0..(table.len() - 1) {
        print!("{:6} ", i);
    }
    print!("   out ");
    println!();
    for (i, a_from) in table.iter().enumerate() {
        if i == table.len() - 1 {
            print!(" in ");
        } else {
            print!("{:3} ", i);
        }

        for &b_to in a_from {
            if b_to.abs() > 0.00001 {
                print!("{:6.3} ", b_to * multiplier);
            } else {
                print!(" _____ ");
            }
        }
        println!();
    }
}

pub fn subtract_table(table_a: &Vec<Vec<f64>>, table_b: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut table_out = Vec::new();
    for i in 0..table_a.len() {
        let mut row = Vec::new();
        for j in 0..table_a.len() {
            row.push(table_a[i][j] - table_b[i][j]);
        }
        table_out.push(row);
    }
    table_out
}
