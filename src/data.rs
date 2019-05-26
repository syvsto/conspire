use core::fmt::Debug;

type QuantitativeType = f64;
type CategoricalType = String;

/// One dimensional data, such as vectors and arrays
pub enum VectorData {
    Quantitative(Vec<QuantitativeType>),
    Categorical(Vec<CategoricalType>),
}

pub trait Plottable: Debug {
    type D;

    fn to_conspire_data(&self) -> Self::D;
}

impl Plottable for Vec<f32> {
    type D = VectorData;

    fn to_conspire_data(&self) -> Self::D {
        VectorData::Quantitative(
            self.iter()
                .map(|x| QuantitativeType::from(*x))
                .collect::<Vec<QuantitativeType>>(),
        )
    }
}

impl Plottable for Vec<f64> {
    type D = VectorData;

    fn to_conspire_data(&self) -> Self::D {
        VectorData::Quantitative(self.to_vec())
    }
}

impl Plottable for Vec<u8> {
    type D = VectorData;

    fn to_conspire_data(&self) -> Self::D {
        VectorData::Quantitative(
            self.iter()
                .map(|x| QuantitativeType::from(*x))
                .collect::<Vec<QuantitativeType>>(),
        )
    }
}

impl Plottable for Vec<i8> {
    type D = VectorData;

    fn to_conspire_data(&self) -> Self::D {
        VectorData::Quantitative(
            self.iter()
                .map(|x| QuantitativeType::from(*x))
                .collect::<Vec<QuantitativeType>>(),
        )
    }
}

impl Plottable for Vec<u32> {
    type D = VectorData;

    fn to_conspire_data(&self) -> Self::D {
        VectorData::Quantitative(
            self.iter()
                .map(|x| QuantitativeType::from(*x))
                .collect::<Vec<QuantitativeType>>(),
        )
    }
}

impl Plottable for Vec<i32> {
    type D = VectorData;

    fn to_conspire_data(&self) -> Self::D {
        VectorData::Quantitative(
            self.iter()
                .map(|x| QuantitativeType::from(*x))
                .collect::<Vec<QuantitativeType>>(),
        )
    }
}

impl Plottable for Vec<String> {
    type D = VectorData;

    fn to_conspire_data(&self) -> Self::D {
        VectorData::Categorical(self.to_vec())
    }
}

impl Plottable for Vec<&'static str> {
    type D = VectorData;

    fn to_conspire_data(&self) -> Self::D {
        VectorData::Categorical(self.iter().map(|x| x.to_string()).collect::<Vec<CategoricalType>>())
    }
}

impl Plottable for &'static str {
    type D = VectorData;

    fn to_conspire_data(&self) -> Self::D {
        VectorData::Categorical(vec![self.to_string()])
    }
}

/// Two-dimensional data, such as matrices
pub enum MatrixData {
    Quantitative(Vec<Vec<QuantitativeType>>),
}

impl Plottable for Vec<Vec<f64>> {
    type D = MatrixData;

    fn to_conspire_data(&self) -> Self::D {
        MatrixData::Quantitative(self.to_vec())
    }
}

impl Plottable for Vec<Vec<i32>> {
    type D = MatrixData;

    fn to_conspire_data(&self) -> Self::D {
        MatrixData::Quantitative(
            self.iter()
                .map(|v| {
                    v.iter()
                        .map(|x| QuantitativeType::from(*x))
                        .collect::<Vec<QuantitativeType>>()
                })
                .collect::<Vec<Vec<QuantitativeType>>>(),
        )
    }
}
