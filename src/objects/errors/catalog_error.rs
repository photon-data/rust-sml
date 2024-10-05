#[derive(Debug)]
pub enum CatalogError {
    InvalidUniqueName(String),
    UnknownObjectType(String),
    InvalidLabel(String),
    InvalidVersionFormat(String),
    InvalidAggressiveAggPromotion(String),
    InvalidBuildSpeculativeAggs(String),

}
