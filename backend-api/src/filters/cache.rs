use super::super::cache::Cache;
use warp::Filter;

pub fn with_cache(
    cache: Cache,
) -> impl Filter<Extract = (Cache,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || cache.clone())
}
