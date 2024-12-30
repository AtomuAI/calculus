
use std::ops::{ Div, Sub };
use num::traits::{ Num, Signed, abs };

pub fn newton_raphson<T, F, DF>( x: T, f: F, df: DF ) -> T
where
    T: Copy + Div<Output = T> + Sub<Output = T>,
    F: Copy + Fn( T ) -> T ,
    DF: Copy + Fn( T ) -> T
{
    x - ( f( x ) / df( x ) )
}

pub struct NewtonRalphsonSolver<T, F, DF>
where
    T: Copy + Div<Output = T> + Sub<Output = T>,
    F: Fn( T ) -> T,
    DF: Fn( T ) -> T
{
    f: F,
    df: DF,
    x: T
}

impl <T, F, DF> NewtonRalphsonSolver<T, F, DF>
where
    T: Copy + Div<Output = T> + Sub<Output = T>,
    F: Copy + Fn( T ) -> T,
    DF: Copy + Fn( T ) -> T
{
    pub fn new( x: T, f: F, df: DF ) -> Self {
        Self {
            f,
            df,
            x
        }
    }

    pub fn step( &mut self ) {
        self.x = newton_raphson( self.x, self.f, self.df );
    }

    pub fn solve( &mut self, tol: T )
    where
        T: Num + Signed + PartialOrd
    {
        let mut x = self.x;
        loop {
            let x_new = newton_raphson( x, self.f, self.df );
            if ( x_new - x ).abs() < tol {
                break;
            }
            x = x_new;
        }
        self.x = x;
    }

    pub fn x( &self ) -> T {
        self.x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_newton_raphson() {
        let mut x: f32 = 0.5;
        for _ in 0..10 {
            println!( "{}", x );
            x = newton_raphson( x, |x| x.powi(2) - 2.0, |x| 2.0 * x );
        }
        println!( "{}", x );
    }

    #[test]
    fn test_newton_raphson_step() {
        let mut solver = NewtonRalphsonSolver::new(
            0.5_f32,
            |x| x.powi(2) - 2.0,
            |x| 2.0 * x
        );
        for _ in 0..10 {
            println!( "{}", solver.x() );
            solver.step();
        }
        println!( "{}", solver.x() );
    }

    #[test]
    fn test_newton_raphson_solver() {
        let mut solver = NewtonRalphsonSolver::new(
            0.5_f32,
            |x| x.powi(2) - 2.0,
            |x| 2.0 * x
        );
        solver.solve( 0.001 );
        println!( "{}", solver.x() );
    }
}
