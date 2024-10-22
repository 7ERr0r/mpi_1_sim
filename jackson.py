#from numpy import random
import math
import random

from Helpers import Event, PacketQueue
from Helpers import EventType

import json


def poisson(lamb):
    return -math.log(1 - random.random()) / lamb


class MpiSimulator:

    def __init__(self, lamb=0.167, mi=0.25, endTimestamp=200.0, numPacketQueues=2, forget = True):
        self.lamb = lamb
        self.mi = mi
        self.numPacketQueues = numPacketQueues
        self.forget = forget

        # array of Event
        self.eventQueue = []
        # array of PacketQueue
        self.packetQueues = [PacketQueue(id) for id in range(numPacketQueues)]

        self.endTimestamp = endTimestamp
        self.currentTimestamp = 0.0

    # def pushEvent(self, timeA, timeService, typ):
    #    self.eventQueue.append(Event(timeA, timeService, typ, self.nowProcessing.queueID))

    def pushNewEvent(self, event):
        self.eventQueue.append(event)

    def pushEvent(self, serviceDuration, typ, queueId):
        self.eventQueue.append(
            Event(timestamp=self.currentTimestamp + serviceDuration, typ=typ, queueId=queueId))

    def handleEndOfService(self, event, forget=True):
        pq = self.packetQueues[event.queueID]
        if pq.packetBufferLen <= 0:
            print("empty??? shouldn't happen %d" % pq.packetBufferLen)
        else:
            packet = pq.popPacket()
            if pq.packetBufferLen > 0:
                tmpEvent = Event(self.currentTime, poisson(
                    self.mi), EventType.ENDOFSERVICE, event.queueID)
                self.pushEvent(tmpEvent)

            newServiceDuration = packet.serviceDuration
            if forget:
                newServiceDuration = poisson(self.lamb)

            if (event.queueID < self.numPacketQueues - 1):
                self.pushEvent(newServiceDuration, EventType.ARRIVAL,
                               event.queueID + 1)

    def generatePacket(self):
        event = Event(poisson(self.lamb), 0, EventType.ARRIVAL, 0)

        return event

    def handlePacketArrival(self, event):
        pq = self.packetQueues[event.queueID]
        l = pq.packetQueueLen()
        pq.pushPacket(event.arrivalPacket)
        if l == 0:
            # start servicing immediately
            event.typ = EventType.ENDOFSERVICE
            if event.serviceDuration > 0:
                pass
            else:
                event.serviceDuration = poisson(self.mi)
            self.pushEvent(event)
        else:
            # service later
            pass

    def simulate(self):
        # no = random.randrange(10)
        # print("Number of packets:", no)
        # for i in range(no):
        #     event = self.generatePacket()
        #     self.pushEvent(event)

        self.pushNewEvent(self.generatePacket())

        while len(self.eventQueue) != 0:  # or len(temp) !=0:

            if self.currentTimestamp > self.endTimestamp:
                break

            self.eventQueue.sort(key=lambda e: e.timestamp)

            # first event
            event = self.eventQueue.pop(0)
            self.currentTimestamp = event.timestamp

            if True:
                e = json.dumps(event.__dict__)
                print("Time of sim: %8.4f %s" % (self.currentTimestamp, e))

            if event.typ == EventType.ENDOFSERVICE:
                self.handleEndOfService(event)
            elif event.typ == EventType.ARRIVAL:
                self.handlePacketArrival(event)

        for q in self.eventQueue:
            e = json.dumps(q.__dict__)
            print(e)


if __name__ == "__main__":
    sim = MpiSimulator(numPacketQueues=5, forget=True)
    sim.simulate()
