import math
import sys


class Bus:
    def __init__(self, busId, lineId, x, y, time):
        self.busId=busId
        self.lineId=lineId
        self.x=x
        self.y=y
        self.time=time
        self.tot_distance=None
        self.tot_time = None
        self.velocity=None

    def set_parameters(self, dict_bus):
        self.tot_distance = dict_bus[self.busId]["tot_distance"]
        self.tot_time = dict_bus[self.busId]["tot_time"]
        self.velocity = self.tot_distance / self.tot_time


def get_distance(bus_list, busId):
    for bus in bus_list:
        if bus.busId == busId:
            return f"{busId} - Total Distance: {bus.tot_distance}"
    return "Missing busId."


def get_avg_velocity(bus_list, lineId):
    total_distance = 0
    total_time = 0

    for bus in bus_list:
        if bus.lineId == lineId:
            total_distance += bus.tot_distance
            total_time += bus.tot_time
            # count += 1 # THINK! AVG_SPEED = SUMMATION OF TOT DISTANCES DIVIDED BY THE SUMMATION OF TOT TIMES

    if total_time > 0:
        avg_speed = total_distance / total_time
        return f"{lineId} - Avg Speed: {avg_speed}"
    else:
        return "Missing lineId."


if __name__ == '__main__':

    if len(sys.argv) == 4:
        fname, flag, value = sys.argv[1:]
        print(fname, flag, value)
    else:
        fname = "ex2_data.txt"
        flag = "-l"
        value = "4"

    bus_list = []
    with (open(fname, "r") as f):
        for line in f:
            busId, lineId, x, y, time = line.split()
            bus_list.append(Bus(int(busId), int(lineId), float(x), float(y), float(time) ) )

    dict_bus = {}
    for bus in bus_list:
        if bus.busId not in dict_bus:
            # default value. tot_distance and tot_time starts from 0 because this is the start point, no gap from coordinate (0,0)!
            dict_bus[bus.busId] = {"last_x":bus.x, "last_y":bus.y, "tot_distance":0, "last_time":bus.time, "tot_time":0}
        else:
            # calculate distance
            tmp_x = bus.x - dict_bus[bus.busId]["last_x"]
            tmp_y = bus.y - dict_bus[bus.busId]["last_y"]
            distance = math.sqrt(tmp_x**2 + tmp_y**2)
            time = bus.time - dict_bus[bus.busId]["last_time"]

            # update values
            dict_bus[bus.busId]["tot_distance"] += distance
            dict_bus[bus.busId]["tot_time"] += time
            dict_bus[bus.busId]["last_x"] = bus.x
            dict_bus[bus.busId]["last_y"] = bus.y
            dict_bus[bus.busId]["last_time"] = bus.time

    [bus.set_parameters(dict_bus) for bus in bus_list]

    if flag == "-b":
        print(get_distance(bus_list, int(value)))
    elif flag == "-l":
        print(get_avg_velocity(bus_list, int(value)))
    else:
        print("No option available.")