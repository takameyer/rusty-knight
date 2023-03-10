use crate::prelude::*;

#[system(for_each)]
#[read_component(WasHit)]
#[read_component(Point)]
#[read_component(Render)]
pub fn effects(ecs: &mut SubWorld, commands: &mut CommandBuffer, #[resource] camera: &Camera) {
    let hit_entities = <(Entity, &WasHit)>::query();
    draw_batch.target(1);

    hit_entities.iter(ecs).for_each(|(entity, _)| {
        let offset = Point::new(camera.left_x, camera.top_y);

        <(&Point, &Render, &WasHit)>::query()
            .iter(ecs)
            .for_each(|(pos, render, _)| {
                draw_batch.set(*pos - offset, ColorPair::new(RED, BLACK), render.glyph);
            });
        draw_batch.submit(5000).expect("Batch error");
        commands.remove(*entity);
    })
}
