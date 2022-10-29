-- Add migration script here
CREATE TABLE IF NOT EXISTS todos
(
    id          INTEGER PRIMARY KEY NOT NULL,
    description TEXT                NOT NULL,
    done        BOOLEAN             NOT NULL DEFAULT 0
);

--
-- TODO X: 参考 https://www.cnblogs.com/endv/p/12129481.html
--

-- 用户表:
CREATE TABLE IF NOT EXISTS user
(
    id       INTEGER PRIMARY KEY NOT NULL,
    user_id  INTEGER             NOT NULL,
    username TEXT                NOT NULL, -- 名称
    status   INTEGER             NOT NULL  -- 状态: 0-未激活, 1-已激活, 2-已过期
);


-- 基础配置:
CREATE TABLE IF NOT EXISTS crypto_exchange
(
    id            INTEGER PRIMARY KEY                 NOT NULL,
    exchange_id   INTEGER                             NOT NULL,
    exchange_name TEXT      DEFAULT ''                NOT NULL, -- 交易所名称
    created_at    TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at    TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL  --

);

-- 配置:
CREATE TABLE IF NOT EXISTS crypto_exchange_settings
(
    id               INTEGER PRIMARY KEY     NOT NULL,
    user_id          INTEGER                 NOT NULL, -- 用户 ID
    exchange_id      INTEGER                 NOT NULL, -- 交易所 ID
    exchange_user_id varchar(256) DEFAULT '' NOT NULL, -- 交易所对应用户 ID
    api_key          varchar(256) DEFAULT '' NOT NULL, -- API key
    permission       TEXT         DEFAULT '' NOT NULL, -- 权限
    label            TEXT         DEFAULT '' NOT NULL, -- 备注
    status           INTEGER      DEFAULT 0  NOT NULL  -- 状态: 0-未激活, 1-已激活, 2-已过期
);


-- 交易记录
CREATE TABLE IF NOT EXISTS trading_records
(
    id          INTEGER PRIMARY KEY                 NOT NULL,
    user_id     INTEGER                             NOT NULL, -- 用户 ID, 允许多帐号
    exchange_id INTEGER                             NOT NULL,
    trade_sn    INTEGER                             NOT NULL,
    trade_pair  Text                                NOT NULL, -- 交易对
    trade_type  INTEGER                             NOT NULL, -- 交易类型: 1: buy, 2: sell
    market_type INTEGER                             NOT NULL, -- 交易市场: 1: spot, 2: futures
    amount      Decimal(10, 2)                      NOT NULL,

    created_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL  --

);
