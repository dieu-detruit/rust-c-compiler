#!/usr/bin/env python3

def push_pop_to_mov(lines):
    prev_is_push = False
    prev_register = ''

    output_lines = []

    for line in lines:
        command = line.split()
        if command[0] == 'push':
            prev_is_push = True
            prev_register = command[1]
            output_lines.append(line)
            continue
        elif command[0] == 'pop':
            if not prev_is_push:
                output_lines.append(line)
                continue

            output_lines.pop()
            if prev_register == command[1]:
                # 同じレジスタからpushしてpopは無意味
                continue
            else:
                output_lines.append('mov {}, {}'.format(
                    command[1], prev_register))
        else:
            output_lines.append(line)

    return output_lines


def indent(lines):
    output_lines = []
    for line in lines:
        if line[0] == '.' or line[len(line)-1] == ':':
            output_lines.append(line)
        else:
            output_lines.append('    ' + line)

    return output_lines


if __name__ == '__main__':
    with open('input/out.S', mode='r') as f:
        lines = [s.strip() for s in f.readlines()]

    lines = push_pop_to_mov(lines)
    lines = push_pop_to_mov(lines)  # push2回pop2回があるので2回かけておく
    lines = indent(lines)

    with open('input/out_optimized.S', mode='w') as f:
        for line in lines:
            f.write(line + '\n')
