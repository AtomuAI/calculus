
pub struct Solver<M, T, U, DF>
where
    M: Fn( T, U, DF ) -> T,
    T: Copy,
    U: Copy,
    DF: Fn( T, U ) -> T
{
    method: M, // method to solve the system
    dsds: DF, // function of change of state: ds_1/ds_0
    state: T, // state of the system
    phantom: std::marker::PhantomData<U>
}

impl <M, T, U, DF> Solver<M, T, U, DF>
where
    M: Copy + Fn( T, U, DF ) -> T,
    T: Copy,
    U: Copy,
    DF: Copy + Fn( T, U ) -> T
{
    pub fn new( method: M, state: T, dsds: DF ) -> Self {
        Self {
            method,
            dsds,
            state,
            phantom: std::marker::PhantomData,
        }
    }

    pub fn step( &mut self, ds: U ) {
        self.state = (self.method)( self.state, ds, self.dsds );
    }

    pub fn solve( &mut self, ds: U, steps: usize ) {
        for _ in 0..steps {
            self.step( ds );
        }
    }

    pub fn state( &self ) -> T {
        self.state
    }

    pub fn state_mut( &mut self ) -> &mut T {
        &mut self.state
    }

    pub fn dsds( &self ) -> DF {
        self.dsds
    }

    pub fn dsds_mut( &mut self ) -> &mut DF {
        &mut self.dsds
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
            |state: ( f32, f32 ), ds: f32, dsds: fn( ( f32, f32 ), f32 ) -> ( f32, f32 )| -> ( f32, f32 ) {
                let dsds = dsds( state, ds );
                ( state.0 + ( ds * dsds.0 ), state.1 + ( ds * dsds.1 ) )
            },
            ( 0.0_f32, 0.0_f32 ), // initial state
            |state: ( f32, f32 ), _: f32| -> ( f32, f32 ) {
                ( 2.0 * state.0, state.1 )
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
            |state: ( f32, f32 ), ds: f32, dsds: fn( ( f32, f32 ), f32 ) -> ( f32, f32 )| -> ( f32, f32 ) {
                let dsds = dsds( state, ds );
                ( state.0 + ( ds * dsds.0 ), state.1 + ( ds * dsds.1 ) )
            },
            ( 0.0_f32, 0.0_f32 ), // initial state
            |state: ( f32, f32 ), _: f32| -> ( f32, f32 ) {
                ( 2.0 * state.0, state.1 )
            }
        );
        let start = std::time::Instant::now();
        solver.solve( 1.0, 10 );
        println!( "Time: {:?}", start.elapsed() );
        println!( "{:?}", solver.state() );
    }
}
