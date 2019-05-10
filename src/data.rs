use core::fmt::Debug;
use num_traits::Zero;

pub trait Ordinal: Debug + Clone + Zero {}
pub trait Nominal: Debug + Clone {}
pub trait Quantitative: Debug + Clone + Zero {}

impl Quantitative for f32 {}
impl Quantitative for f64 {}

impl Nominal for u8 {}
impl Nominal for i8 {}

impl Ordinal for u32 {}
impl Ordinal for usize {}
impl Ordinal for i32 {}
impl Ordinal for isize {}
