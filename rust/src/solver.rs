use std::ops::AddAssign;


pub struct Solver<M, S, T, DF>
where
    M: Fn( S, T, T, DF ) -> S,
    S: Copy,
    T: Copy + AddAssign,
    DF: Fn( S, T ) -> S
{
    method: M, // method to solve the system
    dsdt: DF, // function of change of state: ds_1/ds_0
    state: S, // state of the system
    time: T, // time step
    phantom: std::marker::PhantomData<T>
}

impl <M, S, T, DF> Solver<M, S, T, DF>
where
    M: Copy + Fn( S, T, T, DF ) -> S,
    S: Copy,
    T: Copy + AddAssign,
    DF: Copy + Fn( S, T ) -> S
{
    pub fn new( method: M, state: S, time: T, dsdt: DF ) -> Self {
        Self {
            method,
            dsdt,
            state,
            time,
            phantom: std::marker::PhantomData,
        }
    }

    pub fn step( &mut self, dt: T ) {
        self.state = (self.method)( self.state, self.time, dt, self.dsdt );
        self.time += dt;
    }

    pub fn solve( &mut self, dt: T, steps: usize ) {
        for _ in 0..steps {
            self.step( dt );
        }
    }

    pub fn state( &self ) -> S {
        self.state
    }

    pub fn state_mut( &mut self ) -> &mut S {
        &mut self.state
    }

    pub fn time( &self ) -> T {
        self.time
    }

    pub fn time_mut( &mut self ) -> &mut T {
        &mut self.time
    }

    pub fn dsdt( &self ) -> DF {
        self.dsdt
    }

    pub fn dsdt_mut( &mut self ) -> &mut DF {
        &mut self.dsdt
    }

    pub fn method( &self ) -> M {
        self.method
    }

    pub fn method_mut( &mut self ) -> &mut M {
        &mut self.method
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solver_step() {
        let mut solver = Solver::new(
            |mut state: ( f32, f32 ), time: f32, dt: f32, dsdt: fn( ( f32, f32 ), f32 ) -> ( f32, f32 )| -> ( f32, f32 ) {
                let dsdt = dsdt( state, time );
                state.0 += dt * dsdt.0;
                state.1 += dt * dsdt.1;
                state
            },
            ( 0.0_f32, 0.0_f32 ), // initial state
            0.0_f32, // initial time
            |mut state: ( f32, f32 ), _: f32| -> ( f32, f32 ) {
                state.0 *= 2.0;
                state
            }
        );
        println!( "{:?}", solver.state() );
        for _ in 0..10 {
            solver.step( 1.0 );
            println!( "{:?}", solver.state() );
        }
    }

    #[test]
    fn test_solver_solve() {
        let mut solver = Solver::new(
            |mut state: ( f32, f32 ), time: f32, dt: f32, dsdt: fn( ( f32, f32 ), f32 ) -> ( f32, f32 )| -> ( f32, f32 ) {
                let dsdt = dsdt( state, time );
                state.0 += dt * dsdt.0;
                state.1 += dt * dsdt.1;
                state
            },
            ( 0.0_f32, 0.0_f32 ), // initial state
            0.0_f32, // initial time
            |mut state: ( f32, f32 ), _: f32| -> ( f32, f32 ) {
                state.0 *= 2.0;
                state
            }
        );
        let start = std::time::Instant::now();
        solver.solve( 1.0, 10 );
        println!( "Time: {:?}", start.elapsed() );
        println!( "{:?}", solver.state() );
    }
}
