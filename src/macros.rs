#[macro_export]
macro_rules! vec3 {
    () => (
        $crate::Vec3::new0()
    );
    ($($x:expr),+ $(,)?) => (
        $crate::Vec3::new([$($x as f64),+])
    );
}

#[macro_export]
macro_rules! point3 {
    () => {
        vec3!()
    };
    ($($x:expr),+ $(,)?) => (
        vec3!($($x),+)
    );
}

#[macro_export]
macro_rules! color {
    () => {
        vec3!()
    };
    ($($x:expr),+ $(,)?) => (
        vec3!($($x),+)
    );
}

#[macro_export]
macro_rules! rand_vec3 {
    () => {
        $crate::Vec3::random()
    };
    ($min:expr, $max:expr) => {
        $crate::Vec3::random_range($min, $max)
    };
}
