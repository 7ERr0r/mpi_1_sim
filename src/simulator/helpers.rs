use core::cmp::Ordering;
use std::collections::VecDeque;
use std::*;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct MpiPacket {
    pub service_duration: f64,
    // moment wejscia do koleki
    pub enter_timestamp: f64,
}

impl MpiPacket {
    pub fn new(service_duration: f64) -> Self {
        Self { service_duration,  enter_timestamp: 0.0}
    }

    pub fn real_in_queue_time(&self, exit_timestamp: f64) -> f64 {
        return exit_timestamp - self.enter_timestamp
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum EventType {
    InputArrival,
    Arrival(MpiPacket),
    EndOfService,
}
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Event {
    pub queue_id: usize,
    pub typ: EventType,
}

impl Event {
    pub fn new(typ: EventType, queue_id: usize) -> Self {
        Self { typ, queue_id }
    }
}

// Struktura: czas i zdarzenie
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct TimeEvent {
    pub time: f64,
    pub event: Event,
}

impl Eq for TimeEvent {}

// Implementacja kolejnosci (Order) dla TimeEvent
// zeby sortowac po czasie
impl Ord for TimeEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        other.time.partial_cmp(&self.time).unwrap()
    }
}
impl PartialOrd for TimeEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl TimeEvent {
    // Przesuwa zdarzenie w czasie
    pub fn add_time(&self, duration: f64) -> TimeEvent {
        TimeEvent {
            time: self.time + duration,
            event: self.event,
        }
    }
}

pub struct PacketQueue {
    pub my_queue_id: usize,
    pub queue: VecDeque<MpiPacket>,
}

impl PacketQueue {
    pub fn new(my_queue_id: usize) -> Self {
        Self {
            my_queue_id,
            queue: VecDeque::new(),
        }
    }
    pub fn packet_queue_len(&self) -> usize {
        return self.queue.len();
    }
    pub fn push_packet(&mut self, packet: MpiPacket) {
        self.queue.push_back(packet)
    }
    pub fn pop_packet(&mut self) -> MpiPacket {
        return self
            .queue
            .pop_front()
            .expect("kolejka pakietow jest pusta??");
        //assert!(self.packet_queue.len() >= 0);
    }
    pub fn peek_first_packet(&mut self) -> Option<MpiPacket> {
        return self.queue.get(0).map(|v| *v);
        //assert!(self.packet_queue.len() >= 0);
    }
}
