fn run_program(prog: &[i32]) -> i32 {
    let mut mem = prog.to_vec();

    let mut pc = 0;
    while pc < mem.len() {
        let opcode = mem[pc];
        if opcode == 99 {
            break;
        }

        let addr_a = mem[pc + 1] as usize;
        let addr_b = mem[pc + 2] as usize;
        let addr_d = mem[pc + 3] as usize;

        match opcode {
        1 => mem[addr_d] = mem[addr_a] + mem[addr_b],
        2 => mem[addr_d] = mem[addr_a] * mem[addr_b],
        _ => panic!("invalid opcode: {}", opcode),
        };

        pc += 4;
    }

    mem[0]
}

fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).expect("read line");

    let init = line.trim().split(',')
        .map(str::parse::<i32>)
        .collect::<Result<Vec<i32>, _>>()
        .expect("parse program");

    'outer: for noun in 0..100 {
        for verb in 0..100 {
            let mut prog = init.to_vec();
            prog[1] = noun;
            prog[2] = verb;
            let out = run_program(&prog);
            if out == 19690720 {
                println!("noun {} verb {}", noun, verb);
                break 'outer;
            }
        }
    }
}
