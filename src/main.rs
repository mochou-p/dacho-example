// dacho-example/src/main.rs

#[allow(clippy::wildcard_imports)]
use dacho::*;

fn main() {
    App::new("dacho example")
        .add_system(Schedule::Start,         spawn_circle)
        .add_system(Schedule::Update,        print_dt)
        .add_system(Schedule::Keyboard,      print_key)
        .add_system(Schedule::MouseMovement, print_mouse_position)
        .add_system(Schedule::MouseButton,   print_mouse_button)
        .add_system(Schedule::MouseWheel,    print_mouse_wheel)
        .run();
}

fn spawn_circle((query,): (Query<(WorldComponent,)>,)) {
    query.one().0.borrow().get(|mut world| {
        world.spawn((MeshComponent::circle(V3::ZERO, 0.5),));
    });
}

fn print_dt((query,): (Query<(TimeComponent,)>,)) {
    static mut N: usize = 0;

    let components = query.one();
    let time       = components.0.borrow();

    if unsafe { N } == 100 {
        println!("{}", time.delta);
        unsafe { N = 0; }
    } else {
        unsafe { N += 1; }
    }
}

fn print_key((query,): (Query<(KeyComponent,)>,)) {
    let components = query.one();
    let key        = components.0.borrow();

    println!("{:?}({})", key.code, key.down);
}

fn print_mouse_position((query,): (Query<(MousePositionComponent,)>,)) {
    let components     = query.one();
    let mouse_position = components.0.borrow();

    println!("x: {}, y: {}", mouse_position.x, mouse_position.y);
}

fn print_mouse_button((query,): (Query<(MouseButtonComponent,)>,)) {
    let components   = query.one();
    let mouse_button = components.0.borrow();

    println!("{:?}({})", mouse_button.button, mouse_button.down);
}

fn print_mouse_wheel((query,): (Query<(MouseWheelComponent,)>,)) {
    let components   = query.one();
    let mouse_scroll = components.0.borrow();

    println!("x: {}, y: {}", mouse_scroll.x, mouse_scroll.y);
}

