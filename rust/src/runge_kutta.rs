
use std::ops::{ Add, AddAssign, Mul, Div };
use num::traits::{ Num, Signed, abs };
use num::traits::FromPrimitive;

pub fn runge_kuta_4<T, DF>( x: T, y: T, dx: T, dydx: DF ) -> T
where
    T: Num + FromPrimitive + Copy + Add<Output = T> + Mul<Output = T> + Div<Output = T>,
    DF: Copy + Fn( T, T ) -> T
{
    let k1 = dx * dydx( x, y );
    let k2 = dx * dydx( x + dx * T::from_f32( 0.5 ).unwrap(), y + k1 * T::from_f32( 0.5 ).unwrap() );
    let k3 = dx * dydx( x + dx * T::from_f32( 0.5 ).unwrap(), y + k2 * T::from_f32( 0.5 ).unwrap() );
    let k4 = dx * dydx( x + dx, y + k3 );
    y + ( k1 + ( k2 * T::from_i32( 2 ).unwrap() ) + ( k3 * T::from_i32( 2 ).unwrap() ) + k4 ) / T::from_i32( 6 ).unwrap()
}

pub struct RungeKutta4Solver<T, DF>
where
    T: Copy + Add<Output = T> + Mul<Output = T>,
    DF: Fn( T, T ) -> T
{
    dydx: DF,
    x: T,
    y: T
}

impl <T, DF> RungeKutta4Solver<T, DF>
where
    T: Num + FromPrimitive + Copy + Add<Output = T> + Mul<Output = T> + AddAssign,
    DF: Copy + Fn( T, T ) -> T
{
    pub fn new( x: T, y: T, dydx: DF ) -> Self {
        Self {
            dydx,
            x,
            y
        }
    }

    pub fn step( &mut self, dx: T ) {
        self.y = runge_kuta_4( self.x, self.y, dx, self.dydx );
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
    fn test_euler() {
        let mut y: f32 = 0.0; // y0
        println!( "{}", y );
        for t in 0..10 {
            y = runge_kuta_4( 1.0, t as f32, y, |t, _| 2.0 * t );
            println!( "{}", y );
        }
    }

    #[test]
    fn test_euler_step() {
        let mut solver = RungeKutta4Solver::new(
            0.0_f32, // x0
            0.0_f32, // y0
            |x, _| 2.0 * x
        );
        println!( "{}", solver.x() );
        for _ in 0..10 {
            solver.step( 1.0 );
            println!( "{}", solver.y() );
        }
    }

    #[test]
    fn test_euler_solver() {
        let mut solver = RungeKutta4Solver::new(
            0.0_f32, // x0
            0.0_f32, // y0
            |x, _| 2.0 * x
        );
        let start = std::time::Instant::now();
        solver.solve( 1.0, 10 );
        println!( "Time: {:?}", start.elapsed() );
        println!( "{}", solver.y() );
    }
}
