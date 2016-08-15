from time import sleep
from collections import namedtuple
from ipdb import set_trace
# rules:
# if there are free cells equal or greater than the current block size,
# then the path is legal. What is expected the cells are placed, return
# the first cell position
# if the cells are less than the current block size, but are equal to half,

SAMPLE_BLOCKS = [4, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 4]
UPWARD = [
    (0, 0),
    (0, -1),
    (-1, 0),
    (-1, -1),
    (-2, 0),
    (-2, -1),
    (-3, 0),
    (-3, -1)
]
DOWNWARD = [
    (0, 0),
    (0, -1),
    (1, 0),
    (1, -1),
    (2, 0),
    (2, -1),
    (3, 0),
    (3, -1)
]

TURN_UPWARD = [
    (0, 0),
    (0, -1),
    (1, 0),
    (1, -1),
    (1, -2),
    (1, -3),
    (0, -2),
    (0, -3)
]

TURN_DOWNWARD = [
    (0, 0),
    (0, -1),
    (-1, 0),
    (-1, -1),
    (-1, -2),
    (-1, -3),
    (0, -2),
    (0, -3)
]

LOOKUP_PATHS = [
    {
        'paths': UPWARD,
        'adjust': (-1, 0)
    },
    {
        'paths': DOWNWARD,
        'adjust': (1, 0)
    },
    {
        'paths': [
            (0, 0),
            (-1, 0),
            (0, -1),
            (-1, -1),
            (0, -2),
            (-1, -2),
            (-1, -3),
            (0, -3)
        ],
        'adjust': (1, 0)
    },
]


class Bit(object):
    def __init__(self, x=None, y=None, active=False, is_fixed=False, is_format=False, is_bridge=False, initial=False):
        self.initial = initial
        self.x = x
        self.y = y
        self.active = active
        self.is_fixed = is_fixed
        if x == 6 and y >= 8 and y <= 12:
            self.is_bridge = True
        elif x >= 8 and x <= 12 and y == 6:
            self.is_bridge = True
        else:
            self.is_bridge = False

        if x == 8 and y <= 8:
            self.is_format = True
        elif x == 8 and y >= 13:
            self.is_format = True
        elif x <= 8 and y == 8:
            self.is_format = True
        else:
            self.is_format = False

    def is_not_legal(self):
        if self.active or self.is_format or self.is_fixed:
            return True
        return False

    def is_valid(self):
        if self.active or self.is_format or self.is_fixed or self.is_bridge:
            return False
        return True

    def is_free(self):
        if self.active or self.is_format or self.is_fixed or self.is_bridge:
            return False
        return True

    def __str__(self):
        padding = " "
        if self.initial:
            return padding + 'M'
        if self.is_bridge:
            return padding + 'B'
        if self.is_format:
            return padding + 'F'
        if self.active and self.is_fixed:
            return padding + 'X'
        if self.active:
            return padding + "#"
        if self.is_fixed:
            return padding + "0"
        return "  "


class QR(object):
    def __init__(self, size=21):
        self.size = size
        self.previous = None
        bit_rows = []
        for x in range(0, 21):
            row = []
            for y in range(0, 21):
                if y >= 13 and x <= 7 or y <= 7 and x <= 7 or y <= 7 and x >= 13:
                    bit = Bit(x=x, y=y, is_fixed=True)
                else:
                    bit = Bit(x=x, y=y, is_fixed=False)
                row.append(bit)
            bit_rows.append(row)

        self.bits = bit_rows

    def is_bit(self, x, y):
        if x < 0 or y < 0 or x >= self.size or y >= self.size:
            return False
        bit = self.bits[x][y]
        return bit.is_valid()

    def at_fixed_point(self, x, y):
        try:
            bit = self.bits[x][y]
        except IndexError:
            return False
        return bit.is_bridge

    def is_cell_valid(self, x, y):
        if x < 0 or y < 0 or x >= self.size or y >= self.size:
            return False
        bit = self.bits[x][y]
        return bit.is_free()


    def traverse_upwards(self, x, y, block_size):
        path = []
        if not self.is_bit(x, y + 1):
            floor = y + 1
        else:
            floor = self.size
        while len(path) < block_size:
            if self.is_cell_valid(x + 1, y) and (x + 1, y) not in path:
                path.append((x, y))
                if self.is_cell_valid(x, y - 1):
                    path.append((x, y - 1))
                x += 1
            elif not self.is_cell_valid(x - 1, y - 1) and not self.is_cell_valid(x - 1, y):
                path.append((x, y))
                y -= 1
            elif y + 1 == floor and self.is_cell_valid(x, y) and (x, y) not in path:
                path.append((x, y))
                if self.is_cell_valid(x, y - 1):
                    path.append((x, y - 1))
                    x -= 1

        return path, x, y

    def traverse_downwards(self, x, y, block):
        path = []
        floor = y + 1
        while len(path) < block:
            if self.is_cell_valid(x - 1, y) and (x - 1, y) not in path:
                path.append((x, y))
                if self.is_cell_valid(x, y - 1):
                    path.append((x, y - 1))
                x -= 1
            if not self.is_cell_valid(x + 1, y - 1) and not self.is_cell_valid(x + 1, y):
                path.append((x, y))
                y -= 1
            if y + 1 == floor and self.is_cell_valid(x, y) and (x, y) not in path:
                path.append((x, y))
                if self.is_cell_valid(x, y - 1):
                    path.append((x, y - 1))
                x += 1

        return path, x, y

    def traverse(self, x, y, blocks, direction='up'):
        for block in blocks:
            if not self.is_cell_valid(x - 1, y) and not self.is_cell_valid(x, y + 1):
                path, x, y = self.traverse_downwards(x, y, block)
            if not self.is_cell_valid(x + 1, y) and not self.is_cell_valid(x, y + 1):
                path, x, y = self.traverse_upwards(x, y, block)
            yield path

    def show(self):
        qr = "".join([" " + str(n) if n < 10 else " " + str(n - 10) for n in range(0, 21)])
        qr += "\n"
        for (x, bit_row) in enumerate(self.bits):
            for bit in bit_row:
                qr += str(bit)
            qr += " {x}".format(x=x) + "\n"

        print(qr)


qr = QR()
x, y = 20, 20
blocks = [4, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8]
for path in qr.traverse(x, y, blocks):
    for a, b in path:
        bit = qr.bits[a][b]
        bit.active = True
    bit.is_fixed = True

qr.show()
