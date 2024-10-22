# MPI_1

Symulator grafowy.

## Uruchamianie:


```python

sim = MpiSimulator(lamb=0.167, mi=0.25, endTime=200.0, numPacketQueues = 10)
sim.simulate()
```

## Uruchamianie rust:

```bash
cargo run
```


```rust
let num_packet_queues = 6;
let outside_id = num_packet_queues;
let mut graph = MpiGraph::new(num_packet_queues);
graph.connect(0, 1, 1.0);
graph.connect(1, 2, 1.0);
graph.connect(2, 3, 1.0);
graph.connect(3, 4, 0.7);
graph.connect(4, 5, 1.0);
graph.connect(5, 0, 1.0);

// prawdopodobienstwo:
// wyjscia z sieci: 0.3 (z kolejki id 3)
//  do nast. wezla: 0.7  (do wezla id 4)
graph.connect(3, outside_id, 0.3);

// prawdopodobienstwo:
//      wejscia do sieci: zawsze rowne 1.0, w tym:
// do wezla/kolejki id 0: 0.5
// do wezla/kolejki id 1: 0.5
graph.connect(outside_id, 0, 0.5);
graph.connect(outside_id, 1, 0.5);

graph.sum_probabilities();


let mut sim = MpiSimulator::new(MpiParams {
    lambda: 1.0,
    mi: 0.125,
    forget: false,
    num_packet_queues,
    end_time: 2000.0,
}, graph);
sim.simulate();
```
