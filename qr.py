from time import sleep
from collections import namedtuple
from ipdb import set_trace
# rules:
# if there are free cells equal or greater than the current block size,
# then the path is legal. What is expected the cells are placed, return
# the first cell position
# if the cells are less than the current block size, but are equal to half,

class Bit(object):
    def __init__(self, x=None, y=None, size=21):
        self.initial = False
        self.x = x
        self.y = y
        self.active = False
        if y >= (size - 8) and x <= 8 or y <= 8 and x <= 8 or y <= 8 and x >= (size - 8):
            self.is_fixed_corner = True
        else:
            self.is_fixed_corner = False

        if x == 6 and y >= 8 and y <= (size - 9) or x >= 8 and x <= (size - 9) and y == 6:
            self.is_bridge = True
        else:
            self.is_bridge = False

        self.useable = not self.is_fixed_corner and not self.is_bridge

    def __eq__(self, bit):
        if not bit:
            return False
        return bit.x is self.x and bit.y is self.y

    def mark_cell_active(self):
        self.active = True
        self.useable = False

    def __str__(self):
        padding = "  "
        if self.is_fixed_corner:
            return padding + 'X'
        if self.is_bridge:
            return padding + 'B'
        if self.active and self.is_fixed_corner:
            return padding + 'M'
        if self.active and not self.is_fixed_corner:
            return padding + '#'
        return "   "


class QR(object):
    def __init__(self, size=21):
        self.size = size
        self.previous = None
        bit_rows = []
        for x in range(0, size):
            row = []
            for y in range(0, size):
                bit = Bit(x=x, y=y, size=size)
                row.append(bit)
            bit_rows.append(row)

        self.bits = bit_rows

    def get_cell(self, x, y):
        if x < 0 or y < 0 or x >= self.size or y >= self.size:
            return False
        return self.bits[x][y]

    def is_cell_valid(self, x, y):
        bit = self.get_cell(x, y)
        return bit and bit.useable

    def is_cell_invalid(self, x, y):
        return not self.is_cell_valid(x, y)

    def is_cell_bridge(self, x, y):
        cell = self.get_cell(x, y)
        if not cell:
            return False
        return cell.is_bridge

    def is_cell_fixed_corner(self, x, y):
        cell = self.get_cell(x, y)
        if not cell:
            return False
        return cell.is_fixed_corner

    def traverse_gap(self, x, y, direction='up'):
        if direction == 'up':
            while self.is_cell_bridge(x, y):
                x -= 1
        else:
            while self.is_cell_bridge(x, y):
                x += 1
        return x, y

    def traverse_path_downward(self, x, y, block_size, path=None):
        path = path or []

        while block_size > len(path):
            if x + 1 >= self.size:
                if self.is_cell_valid(x, y - 1) and self.is_cell_invalid(x, y - 2):
                    y -= 1
            if (x, y) not in path:
                path.append((x, y))
            if self.is_cell_bridge(x + 1, y):
                if (x - 1, y - 1) in path and self.is_cell_valid(x, y - 1):
                    y -= 1
                else:
                    x, y = self.traverse_gap(x + 1, y + 1, direction='down')
            elif self.is_cell_invalid(x + 1, y) and (self.is_cell_invalid(x, y + 1) or (x, y + 1) in path):
                path, x, y = self.traverse_path_upward(x, y - 1, block_size, path=path)
            elif self.is_cell_valid(x + 1, y + 1) and (x + 1, y + 1) not in path:
                y += 1
                x += 1
            elif self.is_cell_valid(x, y - 1) and (x, y - 1) not in path:
                y -= 1
            elif self.is_cell_invalid(x, y - 1) and (x + 1, y) not in path:
                x += 1

        return path, x, y

    def traverse_path_upward(self, x, y, block_size, path=None):
        path = path or []

        while block_size > len(path):
            if (x, y) not in path:
                path.append((x, y))
            if self.is_cell_bridge(x - 1, y):
                if (x + 1, y - 1) in path and self.is_cell_valid(x, y - 1):
                    y -= 1
                else:
                    x, y = self.traverse_gap(x - 1, y + 1)
            elif self.is_cell_invalid(x - 1, y) and (self.is_cell_invalid(x, y + 1) or (x, y + 1) in path):
                if self.is_cell_valid(x + 1, y + 1) and (x + 1, y + 1) not in path:
                    path, x, y = self.traverse_path_downward(x, y, block_size, path=path)
                else:
                    y -= 1
            elif self.is_cell_valid(x - 1, y + 1) and (x - 1, y + 1) not in path:
                y += 1
                x -= 1
            elif self.is_cell_valid(x, y - 1) and (x, y - 1) not in path:
                y -= 1
            elif self.is_cell_invalid(x, y - 1) and (x - 1, y) not in path:
                x -= 1

        return path, x, y

    def traverse_upwards(self, x, y, block_size):
        if block_size > 0:
            current_cell = self.get_cell(x, y)

            if not current_cell:
                set_trace()

            current_cell.mark_cell_active()
        if block_size == 0:
            return x, y
        elif block_size == 1:
            current_cell.is_fixed_corner = True
            # if self.is_cell_bridge(x - 1, y):
            #     return self.traverse_gap(x - 1, y + 1)

        if self.is_cell_valid(x - 1, y) and self.is_cell_valid(x, y - 1):
            if self.is_cell_valid(x - 1, y + 1):
                return self.traverse_upwards(x - 1, y + 1, block_size - 1)
            else:
                return self.traverse_upwards(x, y - 1, block_size - 1)

        if self.is_cell_invalid(x - 1, y) and not self.is_cell_bridge(x - 1, y):
            if self.is_cell_valid(x, y - 1) and self.is_cell_valid(x + 1, y - 1):
                return self.traverse_downwards(x, y - 1, block_size - 1)
        if self.is_cell_bridge(x - 1, y):
            if block_size == 1:
                return self.traverse_gap(x - 1, y + 1)
            elif self.is_cell_valid(x + 1, y - 1):
                set_trace()

        return self.traverse_upwards(x, y - 1, block_size - 1)

    def traverse_downwards(self, x, y, block_size):
        if block_size > 0:
            current_cell = self.get_cell(x, y)
            if not current_cell:
                set_trace()
            current_cell.mark_cell_active()

        if block_size == 0:
            return x, y
        elif block_size == 1:
            current_cell.is_fixed_corner = True

        if self.is_cell_valid(x + 1, y) and self.is_cell_valid(x, y - 1):
            if self.is_cell_valid(x + 1, y + 1):
                return self.traverse_downwards(x + 1, y + 1, block_size - 1)
            else:
                return self.traverse_downwards(x, y - 1, block_size - 1)

        if self.is_cell_invalid(x + 1, y) and not self.is_cell_bridge(x + 1, y):
            return self.traverse_upwards(x, y - 1, block_size - 1)
        elif self.is_cell_bridge(x + 1, y):
            if block_size == 1:
                set_trace()
                return self.traverse_gap(x + 1, y)
            elif self.is_cell_valid(x - 1, y - 1):
                set_trace()

        return self.traverse_downwards(x, y - 1, block_size - 1)

    def show(self, path, x, y):
        qr = ""
        for n in range(0, self.size):
            qr += "{0:{width}}".format(n, width=3)
        # qr = " ".join(["  " + str(n) for n in range(0, self.size)])
        qr += "\n"
        for (x, bit_row) in enumerate(self.bits):
            for bit in bit_row:
                qr += "" + str(bit)
            qr += " {x}".format(x=x) + "\n"

        path = [" ({0}, {1}) ".format(x, y) for x, y in path]

        print(qr + "\n" + "".join(path) + "\n" + "x: {x}, y: {y}".format(x=x, y=y))

size = 49
qr = QR(size=size)
x, y = size - 1, size - 1
for t in range(size - 9, size - 4):
    for f in range(size - 9, size - 4):
        qr.bits[t][f].is_bridge = True

def traversing_blind():
    x, y = qr.traverse_upwards(x, y, 4)
    for i in range(0, 49):
        qr.show()
        x, y = qr.traverse_upwards(x, y, 8)

def traversing_path(size):
    x, y = size - 1, size - 1
    # path, x, y = qr.traverse_path_upward(x, y, 4)
    # for a, b in path:
    #     cell = qr.get_cell(a, b)
    #     if cell:
    #         cell.mark_cell_active()
    # cell.is_fixed_corner = True

    for i in range(0, 95):
        path, x, y = qr.traverse_path_upward(x, y, 8)
        print(x, y)
        for a, b in path:
            cell = qr.get_cell(a, b)
            if cell:
                if cell.x == 21 and cell.y == 26:
                    set_trace()
                cell.mark_cell_active()
        cell.is_fixed_corner = True
        qr.show(path, x, y)

traversing_path(size)