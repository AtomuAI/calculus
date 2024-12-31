
use std::ops::{ Add, AddAssign, Mul, Div };
use num::traits::{ Num, Signed, abs };
use num::traits::FromPrimitive;

use crate::solver::Solver;

pub fn runge_kutta_4<S, T, DF>( state: S, time: T, dt: T, dsdt: DF ) -> S
where
    S: Copy + Add<Output = S> + Add<T, Output = S> + Mul<Output = S> + Mul<T, Output = S> + Div<T, Output = S>,
    T: Num + FromPrimitive + Copy + Add<Output = T> + Mul<Output = T> + Div<Output = T> + AddAssign,
    DF: Copy + Fn( S, T ) -> S
{
    let k1 = dsdt( state, time ) * dt;
    let k2 = dsdt( state + k1 * T::from_f32( 0.5 ).unwrap(), time + dt * T::from_f32( 0.5 ).unwrap() ) * dt;
    let k3 = dsdt( state + k2 * T::from_f32( 0.5 ).unwrap(), time + dt * T::from_f32( 0.5 ).unwrap() ) * dt;
    let k4 = dsdt( state + k3, time + dt ) * dt;
    state + ( k1 + ( k2 * T::from_i32( 2 ).unwrap() ) + ( k3 * T::from_i32( 2 ).unwrap() ) + k4 ) / T::from_i32( 6 ).unwrap()
}

pub struct RungeKutta4Solver<S, T, DF>
where
    S: Copy + Add<Output = S> + Mul<Output = S>,
    T: Copy + AddAssign,
    DF: Fn( S, T ) -> S
{
    solver: Solver<fn( S, T, T, DF ) -> S, S, T, DF>,
}

impl <S, T, DF> RungeKutta4Solver<S, T, DF>
where
    S: Copy + Add<Output = S> + Add<T, Output = S> + Mul<Output = S> + Mul<T, Output = S> + Div<T, Output = S>,
    T: Num + FromPrimitive + Copy + Add<Output = T> + Mul<Output = T> + Div<Output = T> + AddAssign,
    DF: Copy + Fn( S, T ) -> S
{
    pub fn new( state: S, time: T, dsdt: DF ) -> Self {
        Self {
            solver: Solver::new( runge_kutta_4, state, time, dsdt )
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
    fn test_runge_kutta_4() {
        let mut state = Vector::<f32, 2>::new( [ 0.0, 0.0 ] );
        println!( "{:?}", state );
        for time in 0..10 {
            state = runge_kutta_4( state, time as f32, 1.0, |mut state, time| {
                state[ 0 ] = 2.0 * time;
                state[ 1 ] = 1.5 * time;
                state
            });
            println!( "{:?}", state );
        }
    }

    #[test]
    fn test_runge_kutta_4_step() {
        let mut solver = RungeKutta4Solver::new(
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
    fn test_runge_kutta_4_solver() {
        let mut solver = RungeKutta4Solver::new(
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
