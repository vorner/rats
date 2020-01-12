use std::ops::DerefMut;

use amethyst::assets::{AssetStorage, Handle, Loader, ProgressCounter};
use amethyst::core::Transform;
use amethyst::core::math::Vector3;
use amethyst::ecs::{Read, ReadExpect, System, SystemData, World};
use amethyst::ecs::prelude::*;
use amethyst::prelude::*;
use amethyst::renderer::{Camera, ImageFormat, SpriteSheet, SpriteSheetFormat, SpriteRender, Texture};
use log::debug;

use crate::maze::Field;

#[derive(Debug)]
pub struct SetSprites;

#[derive(SystemData)]
struct SpriteLoader<'a> {
    loader: ReadExpect<'a, Loader>,
    progress: Write<'a, ProgressCounter>,
    textures: Read<'a, AssetStorage<Texture>>,
    spritesheets: Read<'a, AssetStorage<SpriteSheet>>,
}

impl SpriteLoader<'_> {
    fn load(mut self, img: &str) -> Handle<SpriteSheet> {
        debug!("Loading spritesheets {}", img);
        let texture = self.loader.load(
            format!("{}.png", img),
            // TODO: Better filtering of the image for zoom?
            ImageFormat::default(),
            self.progress.deref_mut(),
            &self.textures,
        );
        self.loader.load(
            format!("{}.ron", img),
            SpriteSheetFormat(texture),
            self.progress.deref_mut(),
            &self.spritesheets,
        )
    }
}

impl SystemDesc<'_, '_, SetSpritesSystem> for SetSprites {
    fn build(self, world: &mut World) -> SetSpritesSystem {
        <SetSpritesSystem as System<'_>>::SystemData::setup(world);

        let maze = world.exec(|d| SpriteLoader::load(d, "maze"));
        let field_reader = world.write_component::<Field>().register_reader();

        let mut camera_trans = Transform::default();
        camera_trans.set_translation_xyz(20.0, 20.0, 1.0);
        let camera = Camera::standard_2d(40.0, 40.0);

        // FIXME: Why do we need this? Shouldn't the bundles get registered before us? :-(
        world.register::<Camera>();
        world.create_entity()
            .with(camera_trans)
            .with(camera)
            .build();

        world.exec(|(e, mut f, mut t): (Entities, WriteStorage<Field>, WriteStorage<Transform>)| {
            let mut transform = Transform::default();
            transform.set_translation_xyz(1.0, 1.0, 0.0);
            transform.set_scale(Vector3::new(0.01, 0.01, 0.01));

            e.build_entity()
                .with(transform.clone(), &mut t)
                .with(Field::Path, &mut f)
                .build();

            transform.set_translation_xyz(1.0, 2.0, 0.0);
            e.build_entity()
                .with(transform, &mut t)
                .with(Field::Wall, &mut f)
                .build();
        });

        SetSpritesSystem {
            maze,
            field_reader,
        }
    }
}

#[derive(SystemData)]
pub struct SetSpriteData<'a> {
    entities: Entities<'a>,
    fields: ReadStorage<'a, Field>,
    sprites: WriteStorage<'a, SpriteRender>,
}

#[derive(Debug)]
pub struct SetSpritesSystem {
    maze: Handle<SpriteSheet>,
    field_reader: ReaderId<ComponentEvent>,
}

impl<'a> System<'a> for SetSpritesSystem {
    type SystemData = SetSpriteData<'a>;
    fn run(&mut self, mut data: SetSpriteData) {
        let mut to_update = BitSet::new();
        let mut to_delete = BitSet::new();
        for field_change in data.fields.channel().read(&mut self.field_reader) {
            match field_change {
                ComponentEvent::Inserted(id) | ComponentEvent::Modified(id) => to_update.add(*id),
                ComponentEvent::Removed(id) => to_delete.add(*id),
            };
        }

        for (entity, field, _) in (&data.entities, &data.fields, &to_update).join() {
            debug!("Setting sprite for entity {:?}", entity);
            let idx = match field {
                Field::Path => 0,
                Field::Wall => 1,
            };
            data.sprites.insert(entity, SpriteRender {
                sprite_number: idx,
                sprite_sheet: self.maze.clone(),
            }).unwrap();
        }

        for (entity, _) in (&data.entities, &to_delete).join() {
            data.sprites.remove(entity);
        }
    }
}
