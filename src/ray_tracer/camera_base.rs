use super::utils::Ray;

pub trait Camera {
    // * The reason why we are generating rays from camera is because this will let us
    // * create different types of cameras. Eg: orthographic cameras
    /**
    Generates a ray from origin to a point on image plane described by u and v. u and v are normalized values for the image plane co-ordinates.

    - `u`: x axis normalized, 0 is left, 1 is right; domain => [0, 1]
    - `v`: y axis normalized, 0 is bottom, 1 is top; domain => [0, 1]

    Returns:

    A Ray
    */
    fn generate_ray(&self, u: f32, v: f32) -> Ray;
}
