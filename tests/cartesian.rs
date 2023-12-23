use aoc_utils::cartesian::*;

#[test]
fn test_point() {
    let a = p2(1, 2);
    assert_eq!(a[0], 1);
    assert_eq!(a[1], 2);
    assert_eq!(a[0], a.x());
    assert_eq!(a[1], a.y());

    let mut b = p2(3, 4);
    assert_eq!(a + b.vector(), p2(4, 6));
    assert_eq!(a + b.vector(), b + a.vector());

    assert_eq!(a - b.vector(), p2(-2, -2));
    assert_eq!(b - a.vector(), p2(2, 2));

    *b.x_ref_mut() += 1;
    assert_eq!(b, p2(4, 4));
}

#[test]
fn vector_constructor() {
    let a = v2(1, 2);
    assert_eq!(a, Vector([1, 2]));
    assert_eq!(a[0], 1);
    assert_eq!(a[1], 2);
    assert_eq!(a[0], a.x());
    assert_eq!(a[1], a.y());
}

#[test]
fn vector_methods() {
    assert_eq!(Vector2::origin(), Vector([0, 0]));
    assert_eq!(Vector2::default(), Vector([0, 0]));
    assert_eq!(Vector2::zero(), Vector([0, 0]));
    assert_eq!(v2(1,2).point(), Point([1, 2]));
    assert_eq!(v2(4,2).square_magnitude(), 4*4 + 2*2);
}

#[test]
fn vector_is_aligned() {
    assert_eq!(v2(1,2).is_aligned(), false);
    assert_eq!(v2(1,0).is_aligned(), true);
    assert_eq!(v2(0,4).is_aligned(), true);
}

#[test]
fn vector_normalize() {
    // TODO: how assert error v2(1,2).normalize()
    assert_eq!(v2(2,0).normalize(), v2(1, 0));
    assert_eq!(v2(0,4).normalize(), v2(0, 1));
    assert_eq!(v2(0,-6).normalize(), v2(0, -1));
}

#[test]
fn vector_ops() {
    assert_eq!(-v2(1,2), v2(-1, -2)); // Neg
    assert_eq!(v2(1,2) + v2(1,2),v2(2, 4)); // Add Vector
    assert_eq!(v2(3,5) - v2(1,2),v2(2, 3)); // Sub Vector

    assert_eq!(v2(3,5) + 1, v2(4, 6)); // Add i64
    assert_eq!(1 + v2(3,5), v2(4, 6)); // Add commutativity
    
    assert_eq!(v2(3,5) * 3, v2(9, 15)); // Mul i64
    assert_eq!(3 * v2(3,5), v2(3,5) * 3); // Mul commutativity

    assert_eq!(v2(3,5) - 1, v2(2,4)); // Sub i64
    assert_eq!(v2(3,5) / 2, v2(1,2)); // Div i64

    let mut v = v2(1, 0);
    v += 1;
    assert_eq!(v, v2(2, 1));
}

#[test]
fn vector_from_char() {
    assert_eq!(Vector2::from('U'), Vector2::UP);
    assert_eq!(Vector2::from('D'), Vector2::DOWN);
    assert_eq!(Vector2::from('L'), Vector2::LEFT);
    assert_eq!(Vector2::from('R'), Vector2::RIGHT);

    assert_eq!(Vector2::from('u'), Vector2::UP);
    assert_eq!(Vector2::from('d'), Vector2::DOWN);
    assert_eq!(Vector2::from('l'), Vector2::LEFT);
    assert_eq!(Vector2::from('r'), Vector2::RIGHT);

    assert_eq!(Vector2::from('^'), Vector2::UP);
    assert_eq!(Vector2::from('v'), Vector2::DOWN);
    assert_eq!(Vector2::from('<'), Vector2::LEFT);
    assert_eq!(Vector2::from('>'), Vector2::RIGHT);
}