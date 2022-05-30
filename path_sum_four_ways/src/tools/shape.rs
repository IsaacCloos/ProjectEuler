pub(crate) enum Shape {
    Quadrilateral(QuadrilateralType),
    Triangle(TriangleType),
}

// revisit name
pub(crate) enum QuadrilateralType {
    Kite,
    Trapezoid,
    Parallelogram,
    IsoscelesTrapezoid,
    Rhombus,
    Rectangle,
    Square
}

// Angle
pub(crate) enum TriangleType {
    Equilateral,
    Isoceles,
    Scalene,
    Acute,
    Right,
    Obtuse
}