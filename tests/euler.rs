use Dzahui::euler_step;

#[test]
fn first_order_ODE() {
    let constant_velocity = |initial_val: &Vec<f32>| {1.0};
    let step = 0.01;
    let mut time: f32 = 0.0;
    let mut pos: f32 = 0.0; // m
    while time <= 10.0 {
        if let [_pos,_time] = &euler_step(vec![pos,time], step, constant_velocity)[..] {
            pos = *_pos;
            time = *_time;
        }; 
    }
    assert_eq!(pos<=10.1,true);
    assert_eq!(pos>=9.9,true);
}

#[test]
fn second_order_ODE_gravity() {
    let gravity: f32 = 9.81; // m/s^2   
    let force_g = |_val: &Vec<f32>| -gravity;
    let step: f32 = 0.01;
    let mut time: f32 = 0.0;
    let mut pos: f32 = 100.0; // m
    let mut vel: f32 = 0.0; // m/s
    while time <= 10.0 { 
        if let [_vel,_pos,_time] = &euler_step(vec![vel,pos,time], step, force_g)[..] {
            vel = *_vel;
            pos = *_pos;
            time = *_time;
        };
    }
    assert_eq!(vel>=-100.0,true);
    assert_eq!(vel<=-97.0,true);
    assert_eq!(pos>=-400.0,true);
    assert_eq!(pos<=-389.0,true);
}

#[test]
fn radioactive_decay_Radium_ODE() {
    let half_life: f32 = 1600.0; //years
    let radioactive_decay_constant: f32 = 2.0_f32.ln()/half_life;
    let decay_function = |val: &Vec<f32>| {
        -radioactive_decay_constant*val.get(0).unwrap() // -lambda * N
    };
    let step: f32 = 0.5; // years
    let mut time: f32 = -1600.0; // years
    let mut quantity: f32 = 1000.0; // parent nuclei
    while time <= 0.0 { // Should give approximately half the original amount of nuclei
        if let [_quantity,_time] = &euler_step(vec![quantity,time], step, decay_function)[..] {
            quantity = *_quantity;
            time = *_time;
        }
    }

    assert_eq!(time<=0.5,true);
    assert_eq!(time>=-0.5,true);
    assert_eq!(quantity>=490.0,true);
    assert_eq!(quantity<=510.0,true);

}