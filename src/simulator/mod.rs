use super::graph::*;
use crate::results::MpiResults;
use rand::prelude::*;
use rand::Rng;
use rand_pcg::Pcg64;
use std::collections::BinaryHeap;
use std::*;
pub mod helpers;
pub mod results;
use helpers::*;

#[derive(Default, Debug, Clone)]
pub struct MpiParams {
    pub lambda: f64,
    pub mi: f64,
    pub forget: bool,
    pub num_packet_queues: usize,
    pub ignore_time: f64,
    pub end_time: f64,
    pub seed: u64,
    pub verbose: bool,
}

pub struct MpiSimulator {
    params: MpiParams,
    pub graph: MpiGraph,
    pub results: MpiResults,

    event_queue: BinaryHeap<TimeEvent>,
    packet_queues: Vec<PacketQueue>,
    current_time: f64,
    pub rng: Pcg64,
}

impl MpiSimulator {
    pub fn new(params: MpiParams, graph: MpiGraph) -> Self {
        let mut sim = Self {
            rng: Pcg64::seed_from_u64(params.seed),
            results: MpiResults::new(&params),
            params,
            event_queue: BinaryHeap::new(),
            current_time: 0.0,
            graph,
            packet_queues: Vec::new(),
        };

        sim.packet_queues = (0..sim.params.num_packet_queues)
            .into_iter()
            .map(|id| PacketQueue::new(id))
            .collect::<Vec<_>>();

        return sim;
    }

    pub fn poisson(&mut self, lamb: f64) -> f64 {
        // ln jest logarytmem naturalnym
        return -(1.0 - self.rng.gen::<f64>()).ln() / lamb;
    }

    fn push_new_event(&mut self, event: Event) {
        self.event_queue.push(TimeEvent {
            time: self.current_time,
            event,
        });
    }
    fn push_event(&mut self, service_duration: f64, typ: EventType, queue_id: usize) {
        self.event_queue.push(TimeEvent {
            time: self.current_time + service_duration,
            event: Event::new(typ, queue_id),
        });
    }

    fn handle_packet_arrival(&mut self, event: &Event) {
        let pq = self.packet_queues.get_mut(event.queue_id);
        // jesli znaleziono kolejke o tym id...
        if let Some(pq) = pq {
            let len = pq.queue.len();
            if let EventType::Arrival(mut arrival_pkt) = event.typ {
                arrival_pkt.enter_timestamp = self.current_time;
                pq.push_packet(arrival_pkt);
            }
            if len == 0 {
                // natychmiast serwisujemy
                let packet = pq.peek_first_packet().unwrap();
                self.start_servicing(event.queue_id, &packet);
            } else {
                // serwisujemy pozniej
            }
        } else {
            // nie znaleziono kolejki, wiec pakiet opuszcza system
        }
    }
    fn start_servicing(&mut self, queue_id: usize, packet: &MpiPacket) {
        // println!("start servicing");
        let service_duration = if self.params.forget {
            self.poisson(self.params.mi)
        } else {
            packet.service_duration
        };
        self.push_event(service_duration, EventType::EndOfService, queue_id);
    }

    fn handle_end_of_service(&mut self, event: &Event) {
        let my_id = event.queue_id;
        let pq = self.packet_queues.get_mut(my_id).unwrap();
        if pq.queue.len() == 0 {
            println!("empty??? shouldn't happen {}", pq.queue.len());
        } else {
            let now_packet = pq.pop_packet();
            let next_queue_id = self.graph.randomize_next_node(&mut self.rng, my_id);
            self.transmit_to_queue(now_packet, my_id, next_queue_id);
        }

        self.try_service_next(my_id);
    }

    fn transmit_to_queue(&mut self, packet: MpiPacket, from_id: usize, to_id: usize) {
        if self.current_time > self.params.ignore_time {
            // mu
            let sd = packet.service_duration;
            // AOI age on information?
            let d = packet.real_in_queue_time(self.current_time);
            self.results.add_transmitted(from_id, to_id, sd);
            self.results.add_delays(from_id, d);
        }
        self.push_event(
            0.0, // natychmiast trafia do nastepnej kolejki
            EventType::Arrival(packet),
            to_id,
        );
    }

    fn try_service_next(&mut self, queue_id: usize) {
        let pq = self.packet_queues.get_mut(queue_id).unwrap();
        let next_packet = pq.peek_first_packet();
        // jesli kolejka niepusta,
        // przetwarzamy kolejny pakiet
        if let Some(next_packet) = next_packet {
            self.start_servicing(queue_id, &next_packet);
        }
    }

    fn handle_input_from_outside_arrival(&mut self, event: &Event) {
        if self.current_time > self.params.end_time {
            return;
        }
        let outside_arrival_queue_id = event.queue_id;
        let next_queue_id = self
            .graph
            .randomize_next_node(&mut self.rng, outside_arrival_queue_id);

        let rand = self.poisson(self.params.lambda);
        self.push_event(rand, EventType::InputArrival, outside_arrival_queue_id);

        let packet = MpiPacket::new(self.poisson(self.params.mi));
        self.push_event(
            0.0, // natychmiast do pierwszej kolejki
            EventType::Arrival(packet),
            next_queue_id,
        );
    }

    fn init_input_arrival_packet_event(&self) -> Event {
        let outside_arrival_queue_id = self.params.num_packet_queues;
        Event::new(EventType::InputArrival, outside_arrival_queue_id)
    }
    pub fn simulate(&mut self) {
        self.push_new_event(self.init_input_arrival_packet_event());

        while self.event_queue.len() > 0 {
            let timeevent = self.event_queue.pop().unwrap();
            self.current_time = timeevent.time;
            // if self.current_time > self.params.end_time {
            //     break;
            // }

            if self.params.verbose {
                let mut comment = "";
                let departure = timeevent.event.queue_id == self.packet_queues.len();
                let input_arrival = timeevent.event.typ == EventType::InputArrival;
                let newline = departure || input_arrival;
                if input_arrival {
                    comment = "arrival / wejscie pakietu do sieci";
                } else if departure {
                    comment = "departure / wyjscie pakietu z sieci";
                }
                if newline {
                    println!();
                }
                println!(
                    "Time of sim: {:8.4} {} {:?}",
                    self.current_time, comment, timeevent.event
                );
                if newline {
                    println!();
                }
            }

            match timeevent.event.typ {
                EventType::EndOfService => self.handle_end_of_service(&timeevent.event),
                EventType::Arrival(_) => self.handle_packet_arrival(&timeevent.event),
                EventType::InputArrival => self.handle_input_from_outside_arrival(&timeevent.event),
            }
        }
        // for e in &self.event_queue {
        //     println!("{:?}", e.event);
        // }
    }

    pub fn debug_remaining_packets(&self) -> usize {
        let mut sum = 0;

        for q in &self.packet_queues {
            sum += q.packet_queue_len();
        }

        sum
    }
}
