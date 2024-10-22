from enum import IntEnum
from json import JSONEncoder


class EventType(IntEnum):
    ARRIVAL = 0
    ENDOFSERVICE = 1


class Event:

    # def __init__(self, time, typ, queueID = 0):
    #     self.time = time
    #     self.typ = typ
    #     self.queueID = queueID

    def __init__(self, timestamp: float = 0, typ: EventType = EventType.ARRIVAL, queueID: int = 0, arrivalPacket = None):
        self.timestamp = timestamp
        self.typ = typ
        self.queueID = queueID
        self.arrivalPacket = arrivalPacket

        if typ == EventType.ARRIVAL:
            assert arrivalPacket != None
        else:
            assert arrivalPacket == None


class MpiPacket:
    def __init__(self, serviceDuration: float = 0.25):
        self.serviceDuration = serviceDuration


class PacketQueue:
    def __init__(self, myQueueID = 0):
        self.myQueueID = myQueueID
        self.packetQueue = []

    def packetQueueLen(self) -> int:
        return len(self.packetQueue)

    def pushPacket(self, packet: MpiPacket):
        self.packetQueue.append(packet)

    def popPacket(self) -> MpiPacket:
        self.packetQueue.pop(0)
        assert len(self.packetQueue) >= 0
