// OgmaValue représente toutes les valeurs manipulables par Ogma.
// Chaque variante correspond à un type natif du langage.
#[derive(Debug)]
pub enum OgmaValue {
    Int(i64),
    String(String),
    Bool(bool),
    Decimal(f64),
    Ognum(String),
}
