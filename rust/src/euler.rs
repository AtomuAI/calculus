
use std::ops::{ Add, AddAssign, Mul };
use num::traits::{ Num, Signed, abs };

fn _euler<T>( y: T, dx: T, dydx: T ) -> T
where
    T: Copy + Add<Output = T> + Mul<Output = T>
{
    y + ( dx * dydx ) // y1 = y0 + ( dx * dy/dx ) -> y1 = y0 + dy
}

pub fn euler<T, DF>( x: T, y: T, dx: T, dydx: DF ) -> T
where
    T: Copy + Add<Output = T> + Mul<Output = T>,
    DF: Copy + Fn( T ) -> T
{
    _euler( y, dx, dydx( x ) )
}

pub struct EulerSolver<T, DF>
where
    T: Copy + Add<Output = T> + Mul<Output = T>,
    DF: Fn( T ) -> T
{
    dydx: DF,
    x: T,
    y: T
}

impl <T, DF> EulerSolver<T, DF>
where
    T: Copy + Add<Output = T> + Mul<Output = T> + AddAssign,
    DF: Copy + Fn( T ) -> T
{
    pub fn new( x: T, y: T, dydx: DF ) -> Self {
        Self {
            dydx,
            x,
            y
        }
    }

    pub fn step( &mut self, dx: T ) {
        self.y = euler( self.x, self.y, dx, self.dydx );
        self.x += dx;
    }

    pub fn solve( &mut self, dx: T, steps: usize ) {
        for _ in 0..steps {
            self.step( dx );
        }
    }

    pub fn x( &self ) -> T {
        self.x
    }

    pub fn y( &self ) -> T {
        self.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euler_direct() {
        let mut y: f32 = 0.0; // y0
        println!( "{}", y );
        for t in 0..10 {
            y = _euler( 1.0, y, 2.0 * ( t as f32 ) );
            println!( "{}", y );
        }
    }

    #[test]
    fn test_euler() {
        let mut y: f32 = 0.0; // y0
        println!( "{}", y );
        for t in 0..10 {
            y = euler( 1.0, t as f32, y, |t| 2.0 * t );
            println!( "{}", y );
        }
    }

    #[test]
    fn test_euler_step() {
        let mut solver = EulerSolver::new(
            0.0_f32, // x0
            0.0_f32, // y0
            |x| 2.0 * x
        );
        println!( "{}", solver.x() );
        for _ in 0..10 {
            solver.step( 1.0 );
            println!( "{}", solver.y() );
        }
    }

    #[test]
    fn test_euler_solver() {
        let mut solver = EulerSolver::new(
            0.0_f32, // x0
            0.0_f32, // y0
            |x| 2.0 * x
        );
        solver.solve( 1.0, 10 );
        println!( "{}", solver.y() );
    }
}
