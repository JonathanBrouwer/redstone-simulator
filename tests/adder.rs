use redstone_simulator::world::World;
use std::fs::File;

#[test]
fn adder_0() {
    let file = File::open("./schematics/adder_0.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("0"));
    assert!(!world.get_probe("1"));
    assert!(!world.get_probe("2"));
    assert!(!world.get_probe("3"));
    assert!(!world.get_probe("4"));
    assert!(!world.get_probe("5"));
    assert!(!world.get_probe("6"));
    assert!(!world.get_probe("7"));
    world.step();
    assert!(!world.get_probe("0"));
    assert!(!world.get_probe("1"));
    assert!(!world.get_probe("2"));
    assert!(!world.get_probe("3"));
    assert!(!world.get_probe("4"));
    assert!(!world.get_probe("5"));
    assert!(!world.get_probe("6"));
    assert!(!world.get_probe("7"));
}

#[test]
fn adder_1() {
    // A = 0101 0101
    // B = 0000 1111
    // Cin = 0
    let file = File::open("./schematics/adder_1.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("0"));
    assert!(!world.get_probe("1"));
    assert!(world.get_probe("2"));
    assert!(!world.get_probe("3"));
    assert!(!world.get_probe("4"));
    assert!(world.get_probe("5"));
    assert!(world.get_probe("6"));
    assert!(!world.get_probe("7"));
    world.step();
    assert!(!world.get_probe("0"));
    assert!(!world.get_probe("1"));
    assert!(world.get_probe("2"));
    assert!(!world.get_probe("3"));
    assert!(!world.get_probe("4"));
    assert!(world.get_probe("5"));
    assert!(world.get_probe("6"));
    assert!(!world.get_probe("7"));
}

#[test]
fn adder_2() {
    // A = 0000 0001
    // B = 0000 0000
    // Cin = 0
    let file = File::open("./schematics/adder_2.schem").unwrap();
    let mut world = World::from(file);

    assert!(world.get_probe("0"));
    world.step();
    assert!(world.get_probe("0"));
}

#[test]
fn adder_3() {
    // A = 0000 0001
    // B = 0000 0001
    // Cin = 0
    let file = File::open("./schematics/adder_3.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("0"));
    world.step();
    assert!(!world.get_probe("0"));
}

#[test]
fn adder_4() {
    // A = 0101 0101
    // B = 0000 1110
    // Cin = 1
    let file = File::open("./schematics/adder_4.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("0"));
    assert!(!world.get_probe("1"));
    assert!(world.get_probe("2"));
    assert!(!world.get_probe("3"));
    assert!(!world.get_probe("4"));
    assert!(world.get_probe("5"));
    assert!(world.get_probe("6"));
    assert!(!world.get_probe("7"));
    world.step();
    assert!(!world.get_probe("0"));
    assert!(!world.get_probe("1"));
    assert!(world.get_probe("2"));
    assert!(!world.get_probe("3"));
    assert!(!world.get_probe("4"));
    assert!(world.get_probe("5"));
    assert!(world.get_probe("6"));
    assert!(!world.get_probe("7"));
}

#[test]
fn adder_5() {
    // A = 0000 0001
    // B = 0000 0001
    // Cin = 1
    let file = File::open("./schematics/adder_5.schem").unwrap();
    let mut world = World::from(file);

    assert!(world.get_probe("0"));
    world.step();
    assert!(world.get_probe("0"));
}

#[test]
fn adder_6() {
    // A = 0000 0000
    // B = 0000 0001
    // Cin = 1
    let file = File::open("./schematics/adder_6h.schem").unwrap();
    let mut world = World::from(file);

    println!("{:?}", world.blocks);

    // assert!(!world.get_probe("t"));
    assert!(world.get_probe("q"));
    world.step();
    // assert!(!world.get_probe("t"));
    assert!(world.get_probe("q"));
}

#[test]
fn adder_7() {
    // A = 0000 0001
    // B = 0000 0001
    // Cin = 0
    let file = File::open("./schematics/adder_7.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("0"));
    assert!(world.get_probe("1"));
    world.step();
    assert!(!world.get_probe("0"));
    assert!(world.get_probe("1"));
}

#[test]
fn torch_test() {
    // A = 0000 0001
    // B = 0000 0001
    // Cin = 0
    let file = File::open("./schematics/torch_test.schem").unwrap();
    let mut world = World::from(file);

    assert!(!world.get_probe("torch_test"));
    world.step();
    assert!(!world.get_probe("torch_test"));
}