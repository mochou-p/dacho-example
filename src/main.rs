// dacho-example/src/main.rs

#[allow(clippy::wildcard_imports)]
use dacho::*;

fn main() {
    App::new("dacho example")
        .add_system(Schedule::Start,  spawn_circle)
        .add_system(Schedule::Update, print_n)
        .run();
}

fn spawn_circle((query,): (Query<(WorldComponent,)>,)) {
    query.one().0.borrow().get(|mut world| {
        world.spawn((
            Mesh::circle(V3::ZERO, 0.5),
        ));

        // temp
        world.spawn((Always,));
    });
}

// forced component because QueryFn currently always expects QueryTuple as argument
fn print_n(_: RunAlways) {
    static mut N: u128 = 0;

    println!("{}", unsafe { N += 1; N });
}

type RunAlways = (Query<(Always,)>,);

struct Always;

