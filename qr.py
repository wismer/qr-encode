from time import sleep

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
        if self.active or self.is_format or self.is_fixed:
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
        if x < 0 or y < 0:
            return False
        try:
            bit = self.bits[x][y]
        except IndexError:
            return False

        return bit.is_valid()

    def distance_to_edge(self, x, y, block_size=4):
        pass
        # i = 0
        # distance_up, distance_down, distance_left = 0, 0, 0
        # while i < block_size + 1:
        #     if self.is_bit(x + i, y):
        #         i += 1

    def is_path_valid(self, x, y, path):
        return all([self.is_bit(x + a, y + b) for a, b in path])

    def draw_pivots_from(self, x, y, block_size=8):
        length = block_size
        if self.is_bit(x, y) and self.bits[x][y].is_bridge:
            if self.is_bit(x + 1, y):
                x, y = x + 1, y
            elif self.is_bit(x - 1, y):
                x, y = x - 1, y
            elif self.is_bit(x, y + 1):
                x, y = x, y + 1
            elif self.is_bit(x, y - 1):
                x, y = x, y - 1

        for path in [UPWARD, TURN_DOWNWARD, DOWNWARD, TURN_UPWARD]:
            if self.is_path_valid(x, y, path[0:length]):
                break

        for a, b in path[0:length]:
            bit = self.bits[x + a][y + b]
            bit.active = True
        bit.is_fixed = True

        if self.is_bit(bit.x - 1, bit.y + 1):
            return bit.x - 1, bit.y + 1
        if self.is_bit(bit.x + 1, bit.y + 1):
            return bit.x + 1, bit.y + 1
        else:
            return (None, None)

    def show(self):
        qr = "".join([" " + str(n) for n in range(0, 21)])
        qr += "\n"
        for (x, bit_row) in enumerate(self.bits):
            for bit in bit_row:
                qr += str(bit)
            qr += " {x}".format(x=x) + "\n"

        print(qr)

    def check_path(self, x, y, block_size=8):
        i = block_size // 2
        path = []
        while i > 0:
            if not self.is_bit(x - i, y):
                # flush saved path and break from loop
                path = []
                i = -4
                break
            else:
                # determine y axis location
                z = 1 if i % 2 == 0 else 0
                path.append((x - i, y - z))
            i -= 1

qr = QR()
x, y = 20, 20
for block in SAMPLE_BLOCKS:
    if x and y:
        x, y = qr.draw_pivots_from(x, y, block_size=block)
    qr.show()