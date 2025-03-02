use redis;
use r2d2;

fn connect_redis_cache() -> redis::RedisResult<r2d2::PooledConnection<redis::Client>> {
    let client = redis::Client::open(String::from("REDIS_CACHE")).unwrap(); // .env variable for redis
    let pool = r2d2::Pool::builder().build(client).unwrap();
    let connection = pool.get().unwrap();
    Ok(connection)
}
