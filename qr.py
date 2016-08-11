from time import sleep

def shift_coord(first, second, index, vertical=True):
    if vertical:
        if index % 2 == 0:
            second -= 1
        else:
            pass


def generate_path_up(x, y):
    path = []
    for i in range(0, 8):
        path.append((x, y))
        if i % 2 == 0:
            y -= 1
        else:
            x -= 1
            y += 1

    return path

def generate_path_left(x, y):
    path = []
    for i in range(0, 8):
        path.append((x, y))
        if i % 2 == 0:
            x -= 1
        else:
            y -= 1
            x += 1

    return path


def generate_path(x, y, length=8, direction=None):
    x_bound, y_bound = direction or (-4, 0)
        

STEPS = [
    (4, 1, 'x'),
    (2, 1, 'x'),
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
        if self.active or self.is_format or self.is_fixed or self.is_bridge:
            return True
        return False

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

    def get_direction(self, x, y):
        directions = []
        for mod_x, mod_y, step, steps in [(-1, 0, -1, 4), (-2, 0), (4, 0), (2, 0), (0, -4), (0, -2)]:
            for i in range(0, step , step)
            try:
                bit = self.bits[x + mod_x][y + mod_y]
            except IndexError:
                bit = False

            if bit and not bit.is_not_legal():
                directions.append((x, mod_x, y, mod_y))

        return directions

    def gen_path(self, x, y, direction='up', block_length=4):
        path = []
        if direction == 'up':
            step, stop = -1, -block_length
            f = lambda c: [(x + c, y), (x + c, y - 1)]
        elif direction == 'left':
            step, stop = 1, block_length
            f = lambda c: [(x, y - c), (x - 1, y - c)]
        else:
            step, stop = 1, block_length
            f = lambda c: [(x + c, y), (x + c, y - 1)]
        for i in range(0, stop, step):
            path += f(i)
        return path

    def set_cells(self, path):
        for x, y in path:
            bit = self.bits[x][y]
            bit.active = True


    def available_paths(self, x=None, y=None, block_size=8):
        if not x:
            path = self.gen_path(20, 20, block_length=block_size // 2)
            x, y = path[-1]
            self.set_cells(path)
            return path, x, y

        for i, j, direction in PATHS:
            try:
                sample_bit = self.bits[x + i][y + j]
            except IndexError:
                pass
            else:
                if sample_bit.is_bridge and direction == 'down':
                    i += 1
                elif sample_bit.is_bridge and direction == 'up':
                    i -= 1
            path = self.gen_path(x + i, y + j, block_length=block_size // 2, direction=direction)
            if self.is_path_legal(path):
                self.set_cells(path)
                self.previous = direction
                x, y = path[-1]
                return path, x, y
        print(x, y)
        return None, x, y

    def is_path_legal(self, path):
        for x, y in path:
            try:
                bit = self.bits[x][y]
            except IndexError:
                return False

            if bit.is_not_legal():
                return False

        return True

    def show(self):
        qr = "".join([" " + str(n) for n in range(0, 21)])
        qr += "\n"
        for (x, bit_row) in enumerate(self.bits):
            for bit in bit_row:
                qr += str(bit)
            qr += " {x}".format(x=x) + "\n"

        print(qr)

qr = QR()
def test_me():
    blocks = [8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8]
    paths = []
    path, x, y = qr.available_paths(block_size=4)
    paths.append(path)
    for block in blocks:
        path, x, y = qr.available_paths(x=x, y=y)
        if path:
            paths.append(path)

    for path in paths:
        xs, xa = path[0]
        a, b = path[-1]
        qr.bits[xs][xa].initial = True
        qr.bits[a][b].is_fixed = True

qr.show()
directions = qr.get_direction(18, 20)
print(directions)
