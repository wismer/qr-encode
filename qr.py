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
        padding = " "
        if self.is_fixed_corner:
            return padding + 'X'
        if self.is_bridge:
            return padding + 'B'
        if self.active and self.is_fixed_corner:
            return padding + 'M'
        if self.active and not self.is_fixed_corner:
            return padding + '#'
        return "  "


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
        try:
            bit = self.bits[x][y]
        except IndexError:
            return False
        return bit

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

    def get_surrounding_cells(self, x, y):
        return [
            self.get_cell(x - 1, y),
            self.get_cell(x, y + 1),
            self.get_cell(x + 1, y),
            self.get_cell(x, y - 1),
        ]

    def traverse_gap(self, x, y, direction='up'):
        if direction == 'up':
            while self.is_cell_bridge(x, y):
                x -= 1
        else:
            while self.is_cell_bridge(x, y):
                x += 1
        return x, y

    def traverse_upwards(self, x, y, block_size):
        current_cell = self.get_cell(x, y)
        current_cell.mark_cell_active()
        block_size -= 1
        if self.is_cell_bridge(x - 1, y) and self.is_cell_bridge(x - 1, y + 1):
            x, y = self.traverse_gap(x - 1, y + 1)
        if block_size == 0:
            return x, y
        elif block_size == 1:
            # so I can see the last cell that was flipped
            current_cell.is_fixed_corner = True

        if self.is_cell_valid(x + 1, y + 1) and self.is_cell_invalid(x, y + 1):
            return self.traverse_upwards(x + 1, y + 1, block_size)

        if self.is_cell_invalid(x, y + 1) and self.is_cell_invalid(x - 1, y):
            return self.traverse_upwards(x, y - 1, block_size)

        if self.is_cell_valid(x - 1, y + 1):
            return self.traverse_upwards(x - 1, y + 1, block_size)

        if self.is_cell_valid(x, y - 1):
            return self.traverse_upwards(x, y - 1, block_size)

    def traverse_downwards(self, x, y, block_size):
        current_cell = self.get_cell(x, y)
        current_cell.mark_cell_active()

        if block_size == 0:
            return x, y

        if self.is_cell_valid(x - 1, y) and self.is_cell_valid(x - 1, y + 1):
            return self.traverse_downwards(x, y - 1, block_size - 1)

        if self.is_cell_invalid(x + 1, y) and self.is_cell_invalid(x, y + 1):
            return self.traverse_downwards(x, y - 1, block_size - 1)

        if self.is_cell_invalid(x + 1, y) and self.is_cell_valid(x - 1, y + 1):
            return self.traverse_downwards(x - 1, y + 1, block_size - 1)

        if self.is_cell_valid(x + 1, y + 1):
            return self.traverse_downwards(x + 1, y + 1, block_size - 1)

        if self.is_cell_valid(x, y - 1) and self.is_cell_valid(x - 1, y):
            return self.traverse_downwards(x, y - 1, block_size - 1)

        if self.is_cell_invalid(x + 1, y) and self.is_cell_invalid(x, y + 1):
            return self.traverse_downwards(x, y - 1, block_size - 1)

        if self.is_cell_valid(x, y - 1):
            return self.traverse_downwards(x, y - 1, block_size - 1)

    def show(self):
        qr = "".join([" " + str(n) if n < 10 else " " + str(n - 10) for n in range(0, 21)])
        qr += "\n"
        for (x, bit_row) in enumerate(self.bits):
            for bit in bit_row:
                qr += str(bit)
            qr += " {x}".format(x=x) + "\n"

        print(qr)

size = 33
qr = QR(size=size)
x, y = size, size
blocks = [4, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8]
for t in range(size - 7, size - 4):
    for f in range(size - 7, size - 4):
        qr.bits[t][f].is_bridge = True
# for path in qr.traverse(x, y, blocks):
#     for bit in path:
#         bit.mark_cell_active()
x, y = qr.traverse_upwards(x - 1, y - 1, 4)
for i in range(0, 13):
    if qr.is_cell_valid(x - 1, y):
        x, y = qr.traverse_upwards(x, y, 8)
    else:
        x, y = qr.traverse_downwards(x, y, 8)
qr.show()
