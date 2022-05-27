use Dzahui::EulerSolver;


#[test]
fn first_order_ode() {

    let new_solver = EulerSolver::new(|initial_val: &[f64;2]| {1.0});

    let step = 0.01;
    let mut time: f64 = 0.0;
    let mut pos: f64 = 0.0; // m
    while time <= 10.0 {
        [pos,time] = new_solver.do_step([pos,time], step);
    }
    assert_eq!(pos<=10.1,true);
    assert_eq!(pos>=9.9,true);
}

#[test]
fn second_order_ode_gravity() {
    
    let new_solver = EulerSolver::new(|_val: &[f64;3]| -9.81);  
    
    let step: f64 = 0.01;
    let mut time: f64 = 0.0;
    let mut pos: f64 = 100.0; // m
    let mut vel: f64 = 0.0; // m/s
    while time <= 10.0 { 
        [vel,pos,time] = new_solver.do_step([vel,pos,time], step);
    }
    assert_eq!(vel>=-100.0,true);
    assert_eq!(vel<=-97.0,true);
    assert_eq!(pos>=-400.0,true);
    assert_eq!(pos<=-389.0,true);
}

#[test]
fn radioactive_decay_radium_ode() {

    let half_life: f64 = 1600.0; //years
    let radioactive_decay_constant: f64 = 2.0_f64.ln()/half_life;
    
    let new_solver = EulerSolver::new(|val: &[f64;2]| 
        {-radioactive_decay_constant*val[0]});

    let step: f64 = 0.5; // years
    let mut time: f64 = -1600.0; // years
    let mut quantity: f64 = 1000.0; // parent nuclei
    while time <= 0.0 { // Should give approximately half the original amount of nuclei
        [quantity,time] = new_solver.do_step([quantity,time], step);
    }

    assert_eq!(time<=0.5,true);
    assert_eq!(time>=-0.5,true);
    assert_eq!(quantity>=490.0,true);
    assert_eq!(quantity<=510.0,true);

}