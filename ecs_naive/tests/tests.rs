use ecs_naive::declare_system;
use ecs_naive::world::World;
use paste::paste;
use itertools::izip;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct TestComponent {
    value: u8,
}

#[test]
fn it_should_add_to_and_entity_a_component_and_update_it_through_a_system() {
    let mut world = World::new();
    world.add_component(0, TestComponent { value: 10 });

    declare_system!(test_fn|TestComponent);
    test_fn(&world, |test_component| {
        test_component.value += 5;
    });

    assert_eq!(
        world.get_component_from_entity::<TestComponent>(0).unwrap(),
        TestComponent { value: 15 }
    );
}

#[test]
fn it_should_override_a_component_if_an_entity_already_had_a_component() {
    let mut world = World::new();
    world.add_component(0, TestComponent { value: 10 });

    world.add_component(0, TestComponent { value: 9 });

    assert_eq!(
        world.get_component_from_entity::<TestComponent>(0).unwrap(),
        TestComponent { value: 9 }
    );
}

#[test]
fn it_should_add_the_same_component_type_to_multiple_entities_and_update_it() {
    let mut world = World::new();
    world.add_component(0, TestComponent { value: 10 });
    world.add_component(1, TestComponent { value: 0 });

    declare_system!(test_fn|TestComponent);
    test_fn(&world, |test_component| {
        test_component.value += 5;
    });

    assert_eq!(
        world.get_component_from_entity::<TestComponent>(0).unwrap(),
        TestComponent { value: 15 }
    );
    assert_eq!(
        world.get_component_from_entity::<TestComponent>(1).unwrap(),
        TestComponent { value: 5 }
    );
}

#[test]
fn it_should_add_to_and_entity_multiple_components_and_update_it_through_a_system() {
    let mut world = World::new();
    world.add_component(0, TestComponent { value: 10 });
    world.add_component(0, 0);
    world.add_component(1, 9);

    declare_system!(test_fn|TestComponent,i32);
    test_fn(&world, |(test_component, i32val)| {
        test_component.value += 5;
        *i32val -= 2;
    });

    assert_eq!(
        world.get_component_from_entity::<TestComponent>(0).unwrap(),
        TestComponent { value: 15 }
    );
    assert_eq!(
        world.get_component_from_entity::<i32>(0).unwrap(),
        -2
    );
    assert_eq!(
        world.get_component_from_entity::<i32>(1).unwrap(),
        9
    );
}

#[test]
fn it_should_delete_a_component() {
    let mut world = World::new();
    world.add_component(0, TestComponent { value: 10 });

    world.delete_component::<TestComponent>(0);

    assert_eq!(
        world.get_component_from_entity::<TestComponent>(0),
        None
    );
}

#[test]
fn it_should_allow_one_to_delete_a_component_from_the_same_entity_multiple_times() {
    let mut world = World::new();
    world.add_component(0, TestComponent { value: 10 });

    world.delete_component::<TestComponent>(0);
    world.delete_component::<TestComponent>(0);

    assert_eq!(
        world.get_component_from_entity::<TestComponent>(0),
        None
    );
}

#[test]
fn it_should_leave_other_components_alone_when_deleting_a_component() {
    let mut world = World::new();
    world.add_component(0, TestComponent { value: 10 });
    world.add_component(0, 9);
    world.add_component(1, TestComponent { value: 11 });

    world.delete_component::<TestComponent>(0);

    assert_eq!(
        world.get_component_from_entity::<TestComponent>(0),
        None
    );
    assert_eq!(
        world.get_component_from_entity::<i32>(0),
        Some(9)
    );
    assert_eq!(
        world.get_component_from_entity::<TestComponent>(1),
        Some(TestComponent { value: 11 })
    );
}