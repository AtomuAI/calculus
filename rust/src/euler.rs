
use std::ops::{ Add, AddAssign, Mul };
use num::traits::{ Num, Signed, abs };

use crate::solver::Solver;

pub fn euler<T, U, DF>( state: T, ds: U, dsds: DF ) -> T
where
    T: Copy + Add<Output = T> + Add<U, Output = T> + Mul<Output = T> + Mul<U, Output = T>,
    U: Copy + Add<Output = U> + Mul<Output = U>,
    DF: Copy + Fn( T, U ) -> T
{
    state + ( dsds( state, ds ) * ds ) // y1 = y0 + ( dx * dy/dx ) -> y1 = y0 + dy
}

pub struct EulerSolver<T, U, DF>
where
    T: Copy + Add<Output = T> + Mul<Output = T>,
    U: Copy,
    DF: Fn( T, U ) -> T
{
    solver: Solver<fn( T, U, DF ) -> T, T, U, DF>,
}

impl <T, U, DF> EulerSolver<T, U, DF>
where
    T: Copy + Add<Output = T> + Add<U, Output = T> + Mul<Output = T> + Mul<U, Output = T>,
    U: Copy + Add<Output = U> + Mul<Output = U>,
    DF: Copy + Fn( T, U ) -> T
{
    pub fn new( state: T, dsds: DF ) -> Self {
        Self {
            solver: Solver::new( euler, state, dsds )
        }
    }

    pub fn step( &mut self, ds: U ) {
        *self.solver.state_mut() = euler( self.solver.state(), ds, self.solver.dsds() );
    }

    pub fn solve( &mut self, ds: U, steps: usize ) {
        for _ in 0..steps {
            self.step( ds );
        }
    }

    pub fn state( &self ) -> T {
        self.solver.state()
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
        for _ in 0..10 {
            state = euler( state, 1.0, |state, ds| [ 2.0 * state[ 1 ], state[ 1 ] + ds ].into() );
            println!( "{:?}", state );
        }
    }

    #[test]
    fn test_euler_step() {
        let mut solver = EulerSolver::new(
            Vector::<f32, 2>::new( [ 0.0, 0.0 ] ),
            |state, ds| [ 2.0 * state[ 1 ], state[ 1 ] + ds ].into()
        );
        println!( "{:?}", solver.state() );
        for _ in 0..10 {
            solver.step( 1.0 );
            println!( "{:?}", solver.state() );
        }
    }

    #[test]
    fn test_euler_solver() {
        let mut solver = EulerSolver::new(
            Vector::<f32, 2>::new( [ 0.0, 0.0 ] ),
            |state, ds| [ 2.0 * state[ 1 ], state[ 1 ] + ds ].into()
        );
        let start = std::time::Instant::now();
        solver.solve( 1.0, 10 );
        println!( "Time: {:?}", start.elapsed() );
        println!( "{:?}", solver.state() );
    }
}
