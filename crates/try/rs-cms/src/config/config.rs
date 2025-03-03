struct AppConfig {
    name: String,
    version: String,

    http: ServerConfig,
    db: DbConfig,
    cache: CacheConfig,
    mq: MQConfig,
}

struct ServerConfig {
    host: String,
    port: String,
}

struct DbConfig {
    url: String,
}

struct CacheConfig {
    url: String,
}

struct MQConfig {
    url: String,
}
