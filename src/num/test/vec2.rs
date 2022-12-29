#[cfg(test)]

use crate::num::ivec2::{IVec2, i2};


#[test]
fn struct_of_i32() {
    let _a = IVec2 {
        x: 1,
        y: 2,
    };

    let _b = IVec2 {
        x: -1,
        y: -2,
    };
}


#[test]
fn easy_constr() {
    let _a = i2(3, 2);
}

#[test]
fn can_equate() {
    let a = i2(1, 2);
    let b = IVec2 { x: 1, y: 2 };

    assert_eq!(a, b);
}

#[test]
fn can_add() {
    let a = i2(5, 4);
    let b = i2(-1, -2);

    let c = a + b;

    assert_eq!(c, i2(4, 2));
}

#[test]
fn can_sub() {
    let a = i2(5, 7);
    let b = i2(-1, -2);

    let c = a - b;

    assert_eq!(c, i2(6, 9));
}

#[test]
fn can_mul() {
    let a = i2(3, 4);
    let b = i2(3, -5);

    let c = a * b;

    assert_eq!(c, i2(9, -20));
}

#[test]
fn can_scale() {
    let a = i2(3, 4);
    let b = 4;

    let c = a * b;

    assert_eq!(c, i2(12, 16));
}