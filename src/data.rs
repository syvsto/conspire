use core::fmt::Debug;

type QuantitativeType = f64;
type CategoricalType = String;

pub enum DataType {
    Quantitative(Vec<QuantitativeType>),
    Categorical(Vec<String>),
}

pub trait Plottable: Debug {
    fn to_conspire_data(&self) -> DataType;
}

impl Plottable for Vec<f32> {
    fn to_conspire_data(&self) -> DataType {
        DataType::Quantitative(self.iter().map(|x| QuantitativeType::from(*x)).collect::<Vec<QuantitativeType>>())
    }
}

impl Plottable for Vec<f64> {
    fn to_conspire_data(&self) -> DataType {
        DataType::Quantitative(self.to_vec())
    }
}

impl Plottable for Vec<u8> {
    fn to_conspire_data(&self) -> DataType {
        DataType::Quantitative(self.iter().map(|x| QuantitativeType::from(*x)).collect::<Vec<QuantitativeType>>())
    }
}

impl Plottable for Vec<i8> {
    fn to_conspire_data(&self) -> DataType {
        DataType::Quantitative(self.iter().map(|x| QuantitativeType::from(*x)).collect::<Vec<QuantitativeType>>())
    }
}

impl Plottable for Vec<u32> {
    fn to_conspire_data(&self) -> DataType {
        DataType::Quantitative(self.iter().map(|x| QuantitativeType::from(*x)).collect::<Vec<QuantitativeType>>())
    }
}

impl Plottable for Vec<i32> {
    fn to_conspire_data(&self) -> DataType {
        DataType::Quantitative(self.iter().map(|x| QuantitativeType::from(*x)).collect::<Vec<QuantitativeType>>())
    }
}

impl Plottable for Vec<String> {
    fn to_conspire_data(&self) -> DataType {
        DataType::Categorical(self.to_vec())
    }
}

impl Plottable for Vec<&'static str> {
    fn to_conspire_data(&self) -> DataType {
        DataType::Categorical(self.iter().map(|x| {
            x.to_string()
        }).collect::<Vec<String>>())
    }
}
