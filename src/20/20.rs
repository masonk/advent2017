#![feature(nll)]

extern crate advent2017 as lib;
use self::lib::particle_parser as parser;
use self::lib::particles;
use self::particles::Particle3;
use self::particles::Vec3;

use std::fs::File;
use std::io::{BufRead, BufReader};
fn parse_input() -> Vec<particles::Particle3> {
    let mut buf = BufReader::new(File::open("src/20/data").unwrap());
    let mut vec = vec![];
    for line in buf.lines() {
        vec.push(parser::parse_Particle(&line.unwrap()).unwrap());
    }
    vec
}

fn part_one() -> usize {
    let particles = parse_input();
    let mut min = &particles[0];
    let mut min_idx = 0;

    for (i, p) in particles[1..].iter().enumerate() {
        let p_a_mag = p.a.magnitude();
        let min_a_mag = min.a.magnitude();

        if p_a_mag < min_a_mag {
            min = p;
            min_idx = i;
        } else if p_a_mag == min_a_mag {
            let p_v_mag = p.v.magnitude();
            let min_v_mag = min.v.magnitude();

            if p_v_mag < min_v_mag {
                min = p;
                min_idx = i;
            } else if p_v_mag == min_v_mag {
                let p_p_mag = p.p.magnitude();
                let min_p_mag = min.p.magnitude();

                if p_p_mag < min_p_mag {
                    min = p;
                    min_idx = i;
                }
            }
        }
    }
    println!("{:?}", min);
    min_idx + 1
}

// --- Part Two ---
// To simplify the problem further, the GPU would like to remove any particles that collide. Particles collide if their positions ever exactly match. Because particles are updated simultaneously, more than two particles can collide at the same time and place. Once particles collide, they are removed and cannot collide with anything else after that tick.

// For example:

// p=<-6,0,0>, v=< 3,0,0>, a=< 0,0,0>
// p=<-4,0,0>, v=< 2,0,0>, a=< 0,0,0>    -6 -5 -4 -3 -2 -1  0  1  2  3
// p=<-2,0,0>, v=< 1,0,0>, a=< 0,0,0>    (0)   (1)   (2)            (3)
// p=< 3,0,0>, v=<-1,0,0>, a=< 0,0,0>

// p=<-3,0,0>, v=< 3,0,0>, a=< 0,0,0>
// p=<-2,0,0>, v=< 2,0,0>, a=< 0,0,0>    -6 -5 -4 -3 -2 -1  0  1  2  3
// p=<-1,0,0>, v=< 1,0,0>, a=< 0,0,0>             (0)(1)(2)      (3)
// p=< 2,0,0>, v=<-1,0,0>, a=< 0,0,0>

// p=< 0,0,0>, v=< 3,0,0>, a=< 0,0,0>
// p=< 0,0,0>, v=< 2,0,0>, a=< 0,0,0>    -6 -5 -4 -3 -2 -1  0  1  2  3
// p=< 0,0,0>, v=< 1,0,0>, a=< 0,0,0>                       X (3)
// p=< 1,0,0>, v=<-1,0,0>, a=< 0,0,0>

// ------destroyed by collision------
// ------destroyed by collision------    -6 -5 -4 -3 -2 -1  0  1  2  3
// ------destroyed by collision------                      (3)
// p=< 0,0,0>, v=<-1,0,0>, a=< 0,0,0>
// In this example, particles 0, 1, and 2 are simultaneously destroyed at the time and place marked X. On the next tick, particle 3 passes through unharmed.

// How many particles are left after all collisions are resolved?

/* discussion

Every time step, update all the particles then check their positions to perform eliminations.

The trick is detecting when there will be no more collisions.

One way to do detect stopping is to stop when all remainin particles are positioned along any dimension in strictly increasing order and are sorted into nondecreasing order for velocity and acceleration. Particles in this condition will never meet on this axis, so they will never meet.

If particle A is with respect to another particle B,
1. Ahead of B,
2. Moving the same or faster than B,
3. Accelerating the same or faster than B

then B will never catch up to A - they will never meet in this axis.

Just check for stopping every 100 steps of simulation.
*/
use std::collections::HashMap;
fn part_two() -> usize {
    let mut particles = HashMap::new();
    let parse = parse_input();
    for (i, p) in parse.into_iter().enumerate() {
        particles.insert(i, p);
    }
    let mut t = 0;
    loop {
        let mut pos: HashMap<String, Vec<usize>> = HashMap::new();
        for (k, v) in particles.iter() {
            let key = format!("{} {} {}", v.p.x, v.p.y, v.p.z);
            if let Some(ref mut vec) = pos.get_mut(&key) {
                vec.push(*k);
            } else {
                pos.insert(key, vec![*k]);
            }
        }
        for v in pos.values() {
            if v.len() > 1 {
                for i in v {
                    particles.remove(i);
                }
            }
        }
        for (_, v) in &mut particles {
            v.update();
        }
        t += 1;
        if t > 1000 {
            // just cheese it
            break;
        }
    }
    particles.len()
}
fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}

/* Discussion:
Every particle traces a path through space according to three single-variable quadratic equations of the form:

x = x_0 + v_{x}t + 1/2a_{x}t^2

for the x, y, and z dimensions

We want to know when two particles are in the same location at the same point in time. For two particles i and j, that occurs precisely whenever there is a solution to this system of equations:

x_i + v_{x_i}t + 1/2a_{x_i}t^2 - x_j - v_{x_j}t - 1/2a_{x_j}t^2 = 0
y_i + v_{y_i}t + 1/2a_{y_i}t^2 - y_j - v_{y_j}t - 1/2a_{y_j}t^2 = 0
z_i + v_{z_i}t + 1/2a_{z_i}t^2 - z_j - v_{z_j}t - 1/2a_{z_j}t^2 = 0

Furthermore the simulation only simulates for integer t >= 0.

First, rearrange these equations and multiply by 2 to give each term integer coefficients.

2(x_i - x_j) + 2(v_{x_i}t - v_{x_j})t + (a_{x_i}t^2 - a_{x_j})t^2 = 0
2(y_i - y_j) + 2(v_{y_i}t - v_{y_j})t + (a_{y_i}t^2 - a_{y_j})t^2 = 0
2(z_i - z_j) + 2(v_{z_i}t - v_{z_j})t + (a_{z_i}t^2 - a_{z_j})t^2 = 0

By the rational root theorem, any integer solution of the first equation will have t be a factor of
2(x_i - x_j)
any integer solution of the second equation has t a factor of
2(y_i - y_j)
likewise for the third, t is a factor of
2(z_i - z_j)

So if t does exist, it is a common factor of each these three terms.

*/
