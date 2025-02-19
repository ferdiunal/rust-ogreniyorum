#[allow(dead_code)]
pub trait Repository<T> {
    fn get_all() -> Vec<T>;
    fn get_by_id(id: &str) -> Option<T>;
    fn create() -> T;
    fn update(id: &str) -> Option<T>;
    fn delete(id: &str) -> bool;
}
