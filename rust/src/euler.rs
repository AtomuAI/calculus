
use std::ops::{ Add, AddAssign, Mul };
use num::traits::{ Num, Signed, abs };

use crate::solver::Solver;

pub fn euler<S, T, DF>( state: S, time: T, dt: T, dsdt: DF ) -> S
where
    S: Copy + Add<Output = S> + Add<T, Output = S> + Mul<Output = S> + Mul<T, Output = S>,
    T: Copy + Add<Output = T> + Mul<Output = T> + AddAssign,
    DF: Copy + Fn( S, T ) -> S
{
    state + ( dsdt( state, time ) * dt ) // s1 = s0 + ( dt * ds/dt ) -> s1 = s0 + ds
}

pub struct EulerSolver<S, T, DF>
where
    S: Copy + Add<Output = S> + Mul<Output = S>,
    T: Copy + AddAssign,
    DF: Fn( S, T ) -> S
{
    solver: Solver<fn( S, T, T, DF ) -> S, S, T, DF>,
}

impl <S, T, DF> EulerSolver<S, T, DF>
where
    S: Copy + Add<Output = S> + Add<T, Output = S> + Mul<Output = S> + Mul<T, Output = S>,
    T: Copy + Add<Output = T> + Mul<Output = T> + AddAssign,
    DF: Copy + Fn( S, T ) -> S
{
    pub fn new( state: S, time: T, dsdt: DF ) -> Self {
        Self {
            solver: Solver::new( euler, state, time, dsdt )
        }
    }

    pub fn step( &mut self, dt: T ) {
        self.solver.step( dt );
    }

    pub fn solve( &mut self, dt: T, steps: usize ) {
        self.solver.solve( dt, steps );
    }

    pub fn state( &self ) -> S {
        self.solver.state()
    }

    pub fn time( &self ) -> T {
        self.solver.time()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use linear_algebra::vector::Vector;

    #[test]
    fn test_euler() {
        let mut state = Vector::<f32, 2>::new( [ 0.0, 0.0 ] );
        println!( "{:?}", state );
        for time in 0..10 {
            state = euler( state, time as f32, 1.0, |mut state, time| {
                state[ 0 ] = 2.0 * time;
                state[ 1 ] = 1.5 * time;
                state
            });
            println!( "{:?}", state );
        }
    }

    #[test]
    fn test_euler_step() {
        let mut solver = EulerSolver::new(
            Vector::<f32, 2>::new( [ 0.0, 0.0 ] ),
            0.0,
            |mut state, time| {
                state[ 0 ] = 2.0 * time;
                state[ 1 ] = 1.5 * time;
                state
            }
        );
        println!( "{:?}", solver.state() );
        for _ in 0..10 {
            println!( "Time: {:?}", solver.time() );
            solver.step( 1.0 );
            println!( "State: {:?}", solver.state() );
        }
    }

    #[test]
    fn test_euler_solver() {
        let mut solver = EulerSolver::new(
            Vector::<f32, 2>::new( [ 0.0, 0.0 ] ),
            0.0,
            |mut state, time| {
                state[ 0 ] = 2.0 * time;
                state[ 1 ] = 1.5 * time;
                state
            }
        );
        let start = std::time::Instant::now();
        solver.solve( 1.0, 10 );
        println!( "Time: {:?}", start.elapsed() );
        println!( "{:?}", solver.state() );
    }
}
