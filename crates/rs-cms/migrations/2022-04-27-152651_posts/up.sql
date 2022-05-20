-- 用户表:
CREATE TABLE user
(
    id         INTEGER PRIMARY KEY AUTO_INCREMENT,
    username   TEXT      NOT NULL,
    email      TEXT      NOT NULL,
    password   TEXT      NOT NULL,
    role       TEXT      NOT NULL,
    is_active  BOOLEAN   NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- 用户信息表:
CREATE TABLE user_profile
(
    id         INTEGER PRIMARY KEY AUTO_INCREMENT,
    user_id    INTEGER   NOT NULL,
    first_name TEXT      NOT NULL,
    last_name  TEXT      NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- 用户地址表:
CREATE TABLE user_address
(
    id         INTEGER PRIMARY KEY AUTO_INCREMENT,
    user_id    INTEGER   NOT NULL,
    street     TEXT      NOT NULL,
    city       TEXT      NOT NULL,
    state      TEXT      NOT NULL,
    zip        TEXT      NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- 商品表:
create table product
(
    id          INTEGER PRIMARY KEY AUTO_INCREMENT,
    product_id  TEXT      NOT NULL, -- 商品ID
    name        TEXT      NOT NULL, -- 商品名称
    description TEXT      NOT NULL, -- 商品描述
    version     TEXT      NOT NULL, -- 版本
    status      TEXT      NOT NULL, -- 状态: 发布, 下架, 删除
    price       INTEGER   NOT NULL, -- 商品单价
    created_at  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- 订单表
create table user_order
(
    id         INTEGER PRIMARY KEY AUTO_INCREMENT,
    order_id   INTEGER   NOT NULL, -- 订单号
    user_id    INTEGER   NOT NULL, -- 用户 ID
    payment_id INTEGER   NOT NULL, -- 支付 ID
    total      INTEGER   NOT NULL, -- 总价
    status     INTEGER   NOT NULL, -- 订单状态: 0-未支付, 1-已支付, 2-已发货, 3-已收货, 4-已完成, 5-已取消
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);


-- 订单明细: kv 表
create table user_order_item
(
    id         INTEGER PRIMARY KEY AUTO_INCREMENT,
    order_id   INTEGER   NOT NULL, -- 订单号
    product_id INTEGER   NOT NULL, -- 商品ID
    price      INTEGER   NOT NULL, -- 单价
    amount     INTEGER   NOT NULL, -- 商品数量
    discount   INTEGER   NOT NULL, -- 折扣
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);


-- 支付表:
create table user_payment
(
    id         INTEGER PRIMARY KEY AUTO_INCREMENT,
    payment_id INTEGER   NOT NULL, -- 支付 ID
    order_id   INTEGER   NOT NULL, -- 订单号
    user_id    INTEGER   NOT NULL, -- 用户 ID
    pay_type   INTEGER   NOT NULL, -- 支付方式: 1-支付宝, 2-微信, 3-银联, 4-网银, 5-其他
    pay_sn     TEXT      NOT NULL, -- 支付流水号
    total      INTEGER   NOT NULL, -- 总价
    status     INTEGER   NOT NULL, -- 订单状态: 0-未支付, 1-已支付, 2-已发货, 3-已收货, 4-已完成, 5-已取消
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);


-- license 表:

CREATE TABLE user_license
(
    id           INTEGER PRIMARY KEY AUTO_INCREMENT,
    license_key  TEXT      NOT NULL,                           -- license key(唯一)
    license_id   TEXT      NOT NULL,                           -- license ID
    user_id      INTEGER   NOT NULL,                           -- 用户 ID
    product_id   TEXT      NOT NULL,                           -- 商品 ID
    active_count INTEGER   NOT NULL,                           -- 激活次数: 可以激活的次数
    used_count   INTEGER   NOT NULL,                           -- 已经使用的次数
    expire_at    TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP, -- 过期时间
    status       INTEGER   NOT NULL,                           -- 状态: 0-未激活, 1-已激活, 2-已过期
    created_at   TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at   TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);


-- 注册码: 激活记录
CREATE TABLE user_license_active_log
(
    id          INTEGER PRIMARY KEY AUTO_INCREMENT,
    license_id  TEXT      NOT NULL, -- license ID
    user_id     INTEGER   NOT NULL, -- 用户 ID
    device_id   TEXT      NOT NULL, -- 设备 ID
    device_type TEXT      NOT NULL, -- 设备类型: 1-手机, 2-平板, 3-电脑
    device_meta TEXT      NOT NULL, -- 设备元数据
    active_at   TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    status      INTEGER   NOT NULL, -- 状态: 0-未激活, 1-已激活, 2-已过期
    created_at  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- 注册码库存:(预分配)
CREATE TABLE license_capacity
(
    id          INTEGER PRIMARY KEY AUTO_INCREMENT,
    license_id  TEXT      NOT NULL, -- license ID
    license_key TEXT      NOT NULL, -- license key
    product_id  TEXT      NOT NULL, -- 商品 ID
    capacity    INTEGER   NOT NULL, -- 容量
    status      INTEGER   NOT NULL, -- 状态: 0-未激活, 1-已激活, 2-已过期
    created_at  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);
