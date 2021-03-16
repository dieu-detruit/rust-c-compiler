#!/usr/bin/env python3

from collections import namedtuple
from itertools import zip_longest
from enum import Enum
Command = namedtuple('Command', ('parsed', 'original'))


class Phase(Enum):
    PUSH = 0
    POP = 1


def push_pop_to_mov(lines):

    output_lines = []

    push_cmd_list = []
    pop_cmd_list = []
    prev_push_arg = None
    phase = Phase.PUSH

    for line in lines:
        parsed = line.split()

        if phase == Phase.PUSH:
            if parsed[0] == 'push':
                push_cmd_list.append(Command(parsed, line))
            elif parsed[0] == 'pop':
                pop_cmd_list.append(Command(parsed, line))
                phase = Phase.POP
            else:
                output_lines.extend([cmd.original for cmd in push_cmd_list])
                output_lines.append(line)
                push_cmd_list.clear()

        elif phase == Phase.POP:
            if parsed[0] == 'pop':
                pop_cmd_list.append(Command(parsed, line))

            else:
                optimized_lines = []
                for push_cmd, pop_cmd in zip_longest(reversed(push_cmd_list), pop_cmd_list):
                    if push_cmd is None:
                        # pop_cmd_listの方が長い時は後ろに追加
                        optimized_lines.append(pop_cmd.original)
                        continue
                    if pop_cmd is None:
                        # push_cmd_listの方が長い時は前に追加
                        optimized_lines.insert(0, push_cmd.original)
                        continue

                    if push_cmd.parsed[1:] == pop_cmd.parsed[1:]:
                        # push x; pop x <=> do nothing
                        continue
                    else:
                        # push x; pop y <=> mov y, x
                        optimized_lines.append('    mov {}, {}\n'.format(
                            ' '.join(pop_cmd.parsed[1:]),
                            ' '.join(push_cmd.parsed[1:])))
                output_lines.extend(optimized_lines)
                push_cmd_list.clear()
                pop_cmd_list.clear()
                output_lines.append(line)
                phase = Phase.PUSH

    return output_lines


def eliminateWasteJump(lines):
    output_lines = []
    for line0, line1 in zip(lines, lines[1:]):
        parsed = line0.split()

        if (parsed[0] == 'jmp') and (line1.strip() == parsed[1] + ':'):
            continue

        output_lines.append(line0)
    output_lines.append(lines[len(lines) - 1])

    return output_lines


if __name__ == '__main__':
    with open('input/out.S', mode='r') as f:
        lines = [s for s in f.readlines()]

    lines = push_pop_to_mov(lines)
    lines = eliminateWasteJump(lines)

    with open('input/out_optimized.S', mode='w') as f:
        for line in lines:
            f.write(line)
