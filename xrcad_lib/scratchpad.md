```rust
use std::collections::HashMap;

/// Common interface for all curve types in CAD applications
pub trait Curve2D {
    /// Get the actual start point where the curve begins
    fn start_point(&self) -> Point2D;
    
    /// Get the actual end point where the curve ends
    fn end_point(&self) -> Point2D;
    
    /// Check if this is a closed curve (start == end)
    fn is_closed(&self) -> bool {
        let start = self.start_point();
        let end = self.end_point();
        (start.x - end.x).abs() < 1e-10 && (start.y - end.y).abs() < 1e-10
    }
    
    /// Get the parameter range (typically 0.0 to 1.0)
    fn parameter_range(&self) -> (f64, f64);
    
    /// Evaluate point on curve at given parameter
    fn evaluate_at(&self, t: f64) -> Point2D;
    
    /// Get curve degree/order
    fn degree(&self) -> usize;
}

pub trait Curve3D {
    fn start_point(&self) -> Point3D;
    fn end_point(&self) -> Point3D;
    fn is_closed(&self) -> bool {
        let start = self.start_point();
        let end = self.end_point();
        (start.x - end.x).abs() < 1e-10 && 
        (start.y - end.y).abs() < 1e-10 && 
        (start.z - end.z).abs() < 1e-10
    }
    fn parameter_range(&self) -> (f64, f64);
    fn evaluate_at(&self, t: f64) -> Point3D;
    fn degree(&self) -> usize;
}

/// Basic 2D point structure
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

/// Basic line segment - most fundamental curve type
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line2D {
    pub start: Point2D,
    pub end: Point2D,
}

impl Curve2D for Line2D {
    fn start_point(&self) -> Point2D { self.start }
    fn end_point(&self) -> Point2D { self.end }
    fn parameter_range(&self) -> (f64, f64) { (0.0, 1.0) }
    fn evaluate_at(&self, t: f64) -> Point2D {
        Point2D {
            x: self.start.x + t * (self.end.x - self.start.x),
            y: self.start.y + t * (self.end.y - self.start.y),
        }
    }
    fn degree(&self) -> usize { 1 }
}

/// Basic 3D point structure
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// Basic line segment in 3D
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line3D {
    pub start: Point3D,
    pub end: Point3D,
}

impl Curve3D for Line3D {
    fn start_point(&self) -> Point3D { self.start }
    fn end_point(&self) -> Point3D { self.end }
    fn parameter_range(&self) -> (f64, f64) { (0.0, 1.0) }
    fn evaluate_at(&self, t: f64) -> Point3D {
        Point3D {
            x: self.start.x + t * (self.end.x - self.start.x),
            y: self.start.y + t * (self.end.y - self.start.y),
            z: self.start.z + t * (self.end.z - self.start.z),
        }
    }
    fn degree(&self) -> usize { 1 }
}

/// Weighted point for rational curves (homogeneous coordinates)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WeightedPoint2D {
    pub x: f64,
    pub y: f64,
    pub weight: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WeightedPoint3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub weight: f64,
}

/// Generic spline - base structure for piecewise polynomial curves
#[derive(Debug, Clone)]
pub struct Spline2D {
    /// Start point of the curve (actual curve endpoint)
    pub start_point: Point2D,
    /// End point of the curve (actual curve endpoint) 
    pub end_point: Point2D,
    /// Control points that influence the curve shape (may not lie on curve)
    pub control_points: Vec<Point2D>,
    /// Degree of the polynomial segments
    pub degree: usize,
    /// Parameter values where segments connect (knot vector)
    pub knots: Vec<f64>,
    /// Whether the spline is closed (forms a loop)
    pub closed: bool,
}

impl Curve2D for Spline2D {
    fn start_point(&self) -> Point2D { self.start_point }
    fn end_point(&self) -> Point2D { self.end_point }
    fn is_closed(&self) -> bool { self.closed }
    fn parameter_range(&self) -> (f64, f64) { 
        (self.knots[0], self.knots[self.knots.len() - 1]) 
    }
    fn evaluate_at(&self, _t: f64) -> Point2D { 
        // Implementation would go here
        self.start_point 
    }
    fn degree(&self) -> usize { self.degree }
}

#[derive(Debug, Clone)]
pub struct Spline3D {
    /// Start point of the curve (actual curve endpoint)
    pub start_point: Point3D,
    /// End point of the curve (actual curve endpoint)
    pub end_point: Point3D,
    /// Control points that influence the curve shape (may not lie on curve)
    pub control_points: Vec<Point3D>,
    pub degree: usize,
    pub knots: Vec<f64>,
    pub closed: bool,
}

/// Bézier curve - uses Bernstein basis functions
#[derive(Debug, Clone)]
pub struct BezierCurve2D {
    /// Start point (curve passes through this)
    pub start_point: Point2D,
    /// End point (curve passes through this)
    pub end_point: Point2D,
    /// Internal control points (handles that don't lie on curve)
    pub control_points: Vec<Point2D>,
}

impl Curve2D for BezierCurve2D {
    fn start_point(&self) -> Point2D { self.start_point }
    fn end_point(&self) -> Point2D { self.end_point }
    fn parameter_range(&self) -> (f64, f64) { (0.0, 1.0) }
    fn evaluate_at(&self, _t: f64) -> Point2D { 
        // Bézier evaluation would go here
        self.start_point 
    }
    fn degree(&self) -> usize { self.control_points.len() + 1 }
}

#[derive(Debug, Clone)]
pub struct BezierCurve3D {
    /// Start point (curve passes through this)
    pub start_point: Point3D,
    /// End point (curve passes through this) 
    pub end_point: Point3D,
    /// Internal control points (handles that don't lie on curve)
    pub control_points: Vec<Point3D>,
}

/// Composite Bézier spline - multiple connected Bézier curves
#[derive(Debug, Clone)]
pub struct BezierSpline2D {
    /// Individual Bézier curve segments
    pub segments: Vec<BezierCurve2D>,
    /// Continuity constraints at segment joints
    pub continuity: ContinuityType,
}

#[derive(Debug, Clone)]
pub struct BezierSpline3D {
    pub segments: Vec<BezierCurve3D>,
    pub continuity: ContinuityType,
}

/// NURBS curve - Non-Uniform Rational B-Spline
#[derive(Debug, Clone)]
pub struct NurbsCurve2D {
    /// Actual start point of the curve
    pub start_point: Point2D,
    /// Actual end point of the curve
    pub end_point: Point2D,
    /// Control points with weights (rational basis)
    pub control_points: Vec<WeightedPoint2D>,
    /// Degree of the B-spline basis functions
    pub degree: usize,
    /// Knot vector (non-uniform spacing allowed)
    pub knots: Vec<f64>,
    /// Whether curve is periodic/closed
    pub periodic: bool,
}

impl Curve2D for NurbsCurve2D {
    fn start_point(&self) -> Point2D { self.start_point }
    fn end_point(&self) -> Point2D { self.end_point }
    fn is_closed(&self) -> bool { self.periodic }
    fn parameter_range(&self) -> (f64, f64) { 
        (self.knots[0], self.knots[self.knots.len() - 1]) 
    }
    fn evaluate_at(&self, _t: f64) -> Point2D { 
        // NURBS evaluation would go here
        self.start_point 
    }
    fn degree(&self) -> usize { self.degree }
}

#[derive(Debug, Clone)]
pub struct NurbsCurve3D {
    pub control_points: Vec<WeightedPoint3D>,
    pub degree: usize,
    pub knots: Vec<f64>,
    pub periodic: bool,
}

/// NURBS surface - tensor product of two NURBS curves
#[derive(Debug, Clone)]
pub struct NurbsSurface {
    /// Control points arranged in a 2D grid
    pub control_points: Vec<Vec<WeightedPoint3D>>,
    /// Degree in u parameter direction
    pub degree_u: usize,
    /// Degree in v parameter direction  
    pub degree_v: usize,
    /// Knot vector in u direction
    pub knots_u: Vec<f64>,
    /// Knot vector in v direction
    pub knots_v: Vec<f64>,
    /// Periodic flags for each direction
    pub periodic_u: bool,
    pub periodic_v: bool,
}

/// Types of continuity between curve segments
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ContinuityType {
    /// C0 - Position continuous (connected)
    Position,
    /// C1 - First derivative continuous (smooth)
    Tangent,
    /// C2 - Second derivative continuous (curvature continuous)
    Curvature,
    /// G1 - Geometric continuity (tangent direction same, magnitude may differ)
    GeometricTangent,
    /// G2 - Geometric curvature continuity
    GeometricCurvature,
}

/// Knot multiplicity and properties
#[derive(Debug, Clone)]
pub struct KnotInfo {
    /// The knot value
    pub value: f64,
    /// How many times this knot value appears
    pub multiplicity: usize,
}

/// Curve evaluation parameters
#[derive(Debug, Clone)]
pub struct EvaluationParams {
    /// Parameter range for evaluation
    pub parameter_range: (f64, f64),
    /// Number of evaluation points
    pub num_points: usize,
    /// Whether to compute derivatives
    pub compute_derivatives: bool,
    /// Maximum derivative order to compute
    pub max_derivative_order: usize,
}

/// Result of curve evaluation
#[derive(Debug, Clone)]
pub struct CurveEvaluation2D {
    /// Parameter values
    pub parameters: Vec<f64>,
    /// Points on curve
    pub points: Vec<Point2D>,
    /// First derivatives (tangent vectors)
    pub first_derivatives: Option<Vec<Point2D>>,
    /// Second derivatives (curvature vectors)
    pub second_derivatives: Option<Vec<Point2D>>,
    /// Curvature values
    pub curvatures: Option<Vec<f64>>,
}

#[derive(Debug, Clone)]
pub struct CurveEvaluation3D {
    pub parameters: Vec<f64>,
    pub points: Vec<Point3D>,
    pub first_derivatives: Option<Vec<Point3D>>,
    pub second_derivatives: Option<Vec<Point3D>>,
    pub curvatures: Option<Vec<f64>>,
}

/// Curve fitting parameters
#[derive(Debug, Clone)]
pub struct FittingParams {
    /// Points to fit curve through
    pub data_points: Vec<Point2D>,
    /// Desired degree of fitted curve
    pub degree: usize,
    /// Tolerance for fitting error
    pub tolerance: f64,
    /// Whether to create closed curve
    pub closed: bool,
    /// Parameterization method
    pub parameterization: ParameterizationMethod,
}

/// Methods for parameterizing data points
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParameterizationMethod {
    /// Uniform spacing in parameter space
    Uniform,
    /// Chord length parameterization
    ChordLength,
    /// Centripetal parameterization
    Centripetal,
}

/// Edge enum - unified representation of all curve types for CAD
#[derive(Debug, Clone)]
pub enum Edge2D {
    Line(Line2D),
    Spline(Spline2D),
    Bezier(BezierCurve2D),
    BezierSpline(BezierSpline2D),
    Nurbs(NurbsCurve2D),
}

#[derive(Debug, Clone)]
pub enum Edge3D {
    Line(Line3D),
    Spline(Spline3D),
    Bezier(BezierCurve3D),
    BezierSpline(BezierSpline3D),
    Nurbs(NurbsCurve3D),
}

impl Curve2D for Edge2D {
    fn start_point(&self) -> Point2D {
        match self {
            Edge2D::Line(line) => line.start_point(),
            Edge2D::Spline(spline) => spline.start_point(),
            Edge2D::Bezier(bezier) => bezier.start_point(),
            Edge2D::BezierSpline(spline) => {
                // First segment's start point
                spline.segments.first().map_or(Point2D { x: 0.0, y: 0.0 }, |s| s.start_point())
            },
            Edge2D::Nurbs(nurbs) => nurbs.start_point(),
        }
    }
    
    fn end_point(&self) -> Point2D {
        match self {
            Edge2D::Line(line) => line.end_point(),
            Edge2D::Spline(spline) => spline.end_point(),
            Edge2D::Bezier(bezier) => bezier.end_point(),
            Edge2D::BezierSpline(spline) => {
                // Last segment's end point
                spline.segments.last().map_or(Point2D { x: 0.0, y: 0.0 }, |s| s.end_point())
            },
            Edge2D::Nurbs(nurbs) => nurbs.end_point(),
        }        
    }
    
    fn parameter_range(&self) -> (f64, f64) {
        match self {
            Edge2D::Line(line) => line.parameter_range(),
            Edge2D::Spline(spline) => spline.parameter_range(),
            Edge2D::Bezier(bezier) => bezier.parameter_range(),
            Edge2D::BezierSpline(_spline) => (0.0, 1.0), // Normalized across all segments
            Edge2D::Nurbs(nurbs) => nurbs.parameter_range(),
        }
    }
    
    fn evaluate_at(&self, t: f64) -> Point2D {
        match self {
            Edge2D::Line(line) => line.evaluate_at(t),
            Edge2D::Spline(spline) => spline.evaluate_at(t),
            Edge2D::Bezier(bezier) => bezier.evaluate_at(t),
            Edge2D::BezierSpline(_spline) => {
                // Would need to map t across segments
                Point2D { x: 0.0, y: 0.0 } // Placeholder
            },
            Edge2D::Nurbs(nurbs) => nurbs.evaluate_at(t),
        }
    }
    
    fn degree(&self) -> usize {
        match self {
            Edge2D::Line(line) => line.degree(),
            Edge2D::Spline(spline) => spline.degree(),
            Edge2D::Bezier(bezier) => bezier.degree(),
            Edge2D::BezierSpline(spline) => {
                // Maximum degree of all segments
                spline.segments.iter().map(|s| s.degree()).max().unwrap_or(0)
            },
            Edge2D::Nurbs(nurbs) => nurbs.degree(),
        }
    }
}

impl Curve3D for Edge3D {
    fn start_point(&self) -> Point3D {
        match self {
            Edge3D::Line(line) => line.start_point(),
            Edge3D::Spline(spline) => spline.start_point(),
            Edge3D::Bezier(bezier) => bezier.start_point(),
            Edge3D::BezierSpline(spline) => {
                spline.segments.first().map_or(Point3D { x: 0.0, y: 0.0, z: 0.0 }, |s| s.start_point())
            },
            Edge3D::Nurbs(nurbs) => nurbs.start_point(),
        }
    }
    
    fn end_point(&self) -> Point3D {
        match self {
            Edge3D::Line(line) => line.end_point(),
            Edge3D::Spline(spline) => spline.end_point(),
            Edge3D::Bezier(bezier) => bezier.end_point(),
            Edge3D::BezierSpline(spline) => {
                spline.segments.last().map_or(Point3D { x: 0.0, y: 0.0, z: 0.0 }, |s| s.end_point())
            },
            Edge3D::Nurbs(nurbs) => nurbs.end_point(),
        }        
    }
    
    fn parameter_range(&self) -> (f64, f64) {
        match self {
            Edge3D::Line(line) => line.parameter_range(),
            Edge3D::Spline(spline) => spline.parameter_range(),
            Edge3D::Bezier(bezier) => bezier.parameter_range(),
            Edge3D::BezierSpline(_spline) => (0.0, 1.0),
            Edge3D::Nurbs(nurbs) => nurbs.parameter_range(),
        }
    }
    
    fn evaluate_at(&self, t: f64) -> Point3D {
        match self {
            Edge3D::Line(line) => line.evaluate_at(t),
            Edge3D::Spline(spline) => spline.evaluate_at(t),
            Edge3D::Bezier(bezier) => bezier.evaluate_at(t),
            Edge3D::BezierSpline(_spline) => {
                Point3D { x: 0.0, y: 0.0, z: 0.0 } // Placeholder
            },
            Edge3D::Nurbs(nurbs) => nurbs.evaluate_at(t),
        }
    }
    
    fn degree(&self) -> usize {
        match self {
            Edge3D::Line(line) => line.degree(),
            Edge3D::Spline(spline) => spline.degree(),
            Edge3D::Bezier(bezier) => bezier.degree(),
            Edge3D::BezierSpline(spline) => {
                spline.segments.iter().map(|s| s.degree()).max().unwrap_or(0)
            },
            Edge3D::Nurbs(nurbs) => nurbs.degree(),
        }
    }
}

/// CAD-specific curve operations that work with any curve type
pub struct CadCurveOperations;

impl CadCurveOperations {
    /// Connect two edges end-to-end (for CAD chaining operations) 
    pub fn can_connect_edges_2d(edge1: &Edge2D, edge2: &Edge2D, tolerance: f64) -> bool {
        let end1 = edge1.end_point();
        let start2 = edge2.start_point();
        let distance = ((end1.x - start2.x).powi(2) + (end1.y - start2.y).powi(2)).sqrt();
        distance <= tolerance
    }
    
    pub fn can_connect_edges_3d(edge1: &Edge3D, edge2: &Edge3D, tolerance: f64) -> bool {
        let end1 = edge1.end_point();
        let start2 = edge2.start_point();
        let distance = ((end1.x - start2.x).powi(2) + (end1.y - start2.y).powi(2) + (end1.z - start2.z).powi(2)).sqrt();
        distance <= tolerance
    }
    
    /// Create a line edge from two points
    pub fn line_2d(start: Point2D, end: Point2D) -> Edge2D {
        Edge2D::Line(Line2D { start, end })
    }
    
    pub fn line_3d(start: Point3D, end: Point3D) -> Edge3D {
        Edge3D::Line(Line3D { start, end })
    }
    
    /// Get edge type as string for debugging/UI
    pub fn edge_type_2d(edge: &Edge2D) -> &'static str {
        match edge {
            Edge2D::Line(_) => "Line",
            Edge2D::Spline(_) => "Spline", 
            Edge2D::Bezier(_) => "Bezier",
            Edge2D::BezierSpline(_) => "BezierSpline",
            Edge2D::Nurbs(_) => "NURBS",
        }
    }
    
    pub fn edge_type_3d(edge: &Edge3D) -> &'static str {
        match edge {
            Edge3D::Line(_) => "Line",
            Edge3D::Spline(_) => "Spline",
            Edge3D::Bezier(_) => "Bezier", 
            Edge3D::BezierSpline(_) => "BezierSpline",
            Edge3D::Nurbs(_) => "NURBS",
        }
    }
    
    /// Convert edge to simple line approximation
    pub fn linearize_2d(edge: &Edge2D) -> Edge2D {
        Edge2D::Line(Line2D {
            start: edge.start_point(),
            end: edge.end_point(),
        })
    }
    
    pub fn linearize_3d(edge: &Edge3D) -> Edge3D {
        Edge3D::Line(Line3D {
            start: edge.start_point(),
            end: edge.end_point(),
        })
    }
}

/// Geometric queries and intersections
#[derive(Debug, Clone)]
pub struct IntersectionResult {
    /// Parameter values on first curve
    pub parameters_curve1: Vec<f64>,
    /// Parameter values on second curve  
    pub parameters_curve2: Vec<f64>,
    /// Intersection points
    pub intersection_points: Vec<Point2D>,
    /// Intersection types (crossing, tangent, etc.)
    pub intersection_types: Vec<IntersectionType>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IntersectionType {
    /// Curves cross at this point
    Crossing,
    /// Curves are tangent at this point
    Tangent,
    /// Curves overlap in a segment
    Overlap,
}

/// Curve analysis results
#[derive(Debug, Clone)]
pub struct CurveAnalysis {
    /// Total arc length
    pub arc_length: f64,
    /// Bounding box
    pub bounding_box: (Point2D, Point2D), // min, max
    /// Points of maximum curvature
    pub curvature_extrema: Vec<(f64, f64)>, // parameter, curvature value
    /// Inflection points (zero curvature)
    pub inflection_points: Vec<f64>, // parameter values
    /// Self-intersection points
    pub self_intersections: Vec<Point2D>,
}
```