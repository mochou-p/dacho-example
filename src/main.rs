// dacho-example/src/main.rs

#[allow(clippy::wildcard_imports)]
use dacho::*;

fn main() {
    let mut app = App::new("dacho example");

    app.add_system(start);

    app.run();
}

fn start((query,): (Query<(WorldComponent,)>,)) {
    query.one().0.borrow().get(|mut world| {
        world.spawn((
            Mesh::circle(V3::ZERO, 0.5),
        ));
    });
}

