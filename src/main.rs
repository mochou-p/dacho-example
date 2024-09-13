// dacho-example/src/main.rs

#[allow(clippy::wildcard_imports)]
use dacho::*;

fn main() {
    let mut app = App::new("dacho example");

    app.add_system(start);

    app.run();
}

fn start((query,): (Query<(WorldComponent,)>,)) {
    let world_comp = query.one().0.borrow().get();
    let mut world  = world_comp.borrow_mut();

    world.spawn((
        Mesh::circle(V3::ZERO, 0.5),
    ));
}

