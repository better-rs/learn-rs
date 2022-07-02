-- 商品订单表:
USE cms;

CREATE TABLE `order_template`
(
    `id`               int(11) unsigned NOT NULL AUTO_INCREMENT COMMENT '自增主键(pk)',
    `mid`              int(11) unsigned NOT NULL DEFAULT '0' COMMENT 'up主的mid',
    `order_no`         varchar(128) CHARACTER SET utf8mb4 NOT NULL DEFAULT '' COMMENT 'UP主兑换订单号(uk)',
    `merchant_id`      int(11) unsigned NOT NULL DEFAULT '0' COMMENT '商户ID',
    `product_id`       int(11) unsigned NOT NULL DEFAULT '0' COMMENT '商品ID',
    `product_type`     tinyint(4) unsigned NOT NULL DEFAULT '0' COMMENT '商品类型: 0=未定义, 1=虚拟商品, 2=实物商品',
    `product_sku`      int(11) unsigned NOT NULL DEFAULT '0' COMMENT '商品SKU',
    `product_title`    varchar(128) CHARACTER SET utf8mb4 NOT NULL DEFAULT '' COMMENT '商品标题',
    `product_num`      int(11) unsigned NOT NULL DEFAULT '0' COMMENT '商品数量',
    `product_price`    decimal(10, 2)                     NOT NULL DEFAULT '0.0' COMMENT '商品单价',
    `product_discount` decimal(10, 2)                     NOT NULL DEFAULT '1.0' COMMENT '商品折扣',
    `cost`             decimal(10, 2)                     NOT NULL DEFAULT '0.00' COMMENT '消耗电池总数',
    `status`           int(11) NOT NULL DEFAULT '0' COMMENT '订单状态: -2=退款, -1=失败(结束), 0=未定义, 1=成功(结束), 2=pending',
    `ver`              int(11) NOT NULL DEFAULT '1' COMMENT '版本号，用于乐观锁',
    `ctime`            timestamp                          NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `mtime`            timestamp                          NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '修改时间',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_order_no` (`order_no`),
    KEY                `ix_mid` (`mid`),
    KEY                `ix_ctime` (`ctime`),
    KEY                `ix_mtime` (`mtime`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='订单表';

-- ################################################################################################################

-- 用户表:
CREATE TABLE `user`
(
    `id`         int(11) unsigned NOT NULL AUTO_INCREMENT COMMENT '自增主键(pk)',
    `user_id`    int(11) unsigned NOT NULL DEFAULT '0' COMMENT 'User ID',
    `username`   varchar(128) CHARACTER SET utf8mb4 NOT NULL DEFAULT '' COMMENT '用户名',
    `email`      varchar(128) CHARACTER SET utf8mb4 NOT NULL DEFAULT '' COMMENT '邮箱',
    `password`   varchar(128) CHARACTER SET utf8mb4 NOT NULL DEFAULT '' COMMENT '密码',
    `role`       int(11) NOT NULL DEFAULT '1' COMMENT '角色状态: 0=未定义, 1=普通用户, 2=管理员, 3=超级管理员',

    -- common fields:
    `status`     int(11) NOT NULL DEFAULT '-1' COMMENT '用户状态: -4=删除, -3=永久封禁, -2=暂停使用, -1=未激活, 0=未定义, 1=激活',
    `created_at` TIMESTAMP                          NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` TIMESTAMP                          NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '修改时间',
    -- index fields:
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_user_id` (`user_id`),
    KEY          `idx_created_at` (`created_at`), -- 建索引
    KEY          `idx_updated_at` (`updated_at`)  -- 建索引
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='用户表';


-- 用户登录方式表: (多对1)
CREATE TABLE `user_auth`
(
    `id`            int(11) unsigned NOT NULL AUTO_INCREMENT COMMENT '自增主键(pk)',
    `auth_id`       varchar(256) CHARACTER SET utf8mb4 NOT NULL DEFAULT '' COMMENT ' 登录鉴权方式:邮箱 ID/ 手机号/ 用户名 等',
    `auth_provider` tinyint(4) unsigned NOT NULL DEFAULT '0' COMMENT '登录鉴权方式: 0=未定义, 1=邮箱, 2=手机号, 3=用户名, 4=OAuth',
    `user_id`       int(11) unsigned NOT NULL DEFAULT '0' COMMENT 'User ID',

    -- common fields:
    `status`        int(11) NOT NULL DEFAULT '1' COMMENT '登录方式状态: -4=删除, -3=永久封禁, -2=暂停使用, -1=未激活, 0=未定义, 1=正常使用',
    `created_at`    TIMESTAMP                          NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at`    TIMESTAMP                          NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '修改时间',
    -- index fields:
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_auth_id` (`auth_id`),             -- 建索引: 登录鉴权方式/唯一
    KEY             `idx_created_at` (`created_at`), -- 建索引
    KEY             `idx_updated_at` (`updated_at`)  -- 建索引
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='用户登录方式表';


-- 登录历史:
CREATE TABLE `user_auth_log`
(
    `id`         int(11) unsigned NOT NULL AUTO_INCREMENT COMMENT '自增主键(pk)',
    `user_id`    int(11) unsigned NOT NULL DEFAULT '0' COMMENT 'User ID',
    `ip`         varchar(128) CHARACTER SET utf8mb4 NOT NULL DEFAULT '' COMMENT 'ip',
    `device`     varchar(128) CHARACTER SET utf8mb4 NOT NULL DEFAULT '' COMMENT 'auth device',
    `meta`       varchar(128) CHARACTER SET utf8mb4 NOT NULL DEFAULT '' COMMENT 'auth meta',

    -- common fields:
    `created_at` TIMESTAMP                          NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` TIMESTAMP                          NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '修改时间',
    -- index fields:
    PRIMARY KEY (`id`),
    KEY          `idx_created_at` (`created_at`), -- 建索引
    KEY          `idx_updated_at` (`updated_at`)  -- 建索引
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='用户登录历史表';


-- 用户基本信息表:
CREATE TABLE `user_profile`
(
    `id`         int(11) unsigned NOT NULL AUTO_INCREMENT COMMENT '自增主键(pk)',
    `user_id`    int(11) unsigned NOT NULL DEFAULT '0' COMMENT 'User ID',
    `username`   varchar(128) CHARACTER SET utf8mb4 NOT NULL DEFAULT '' COMMENT '用户名',
    `email`      varchar(128) CHARACTER SET utf8mb4 NOT NULL DEFAULT '' COMMENT '邮箱',
    `mobile`     varchar(128) CHARACTER SET utf8mb4 NOT NULL DEFAULT '' COMMENT '手机号: 国家码+手机号',
    --
    `nickname`   varchar(128) CHARACTER SET utf8mb4 NOT NULL DEFAULT '' COMMENT '昵称',
    `avatar`     varchar(128) CHARACTER SET utf8mb4 NOT NULL DEFAULT '' COMMENT '头像',
    --
    `real_name`  varchar(128) CHARACTER SET utf8mb4 NOT NULL DEFAULT '' COMMENT '真实姓名',
    `first_name` varchar(128) CHARACTER SET utf8mb4 NOT NULL DEFAULT '' COMMENT '名',
    `last_name`  varchar(128) CHARACTER SET utf8mb4 NOT NULL DEFAULT '' COMMENT '姓',
    -- common fields:
    `created_at` TIMESTAMP                          NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` TIMESTAMP                          NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '修改时间',
    -- index fields:
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_user_id` (`user_id`),
    KEY          `idx_created_at` (`created_at`), -- 建索引
    KEY          `idx_updated_at` (`updated_at`)  -- 建索引
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='用户信息表';


-- 用户第三方账号表:
CREATE TABLE `user_3rd_account`
(
    `id`            int(11) unsigned NOT NULL AUTO_INCREMENT COMMENT '自增主键(pk)',
    `user_id`       int(11) unsigned NOT NULL DEFAULT '0' COMMENT 'User ID',
    `provider`      varchar(128) CHARACTER SET utf8mb4 NOT NULL DEFAULT '' COMMENT '第三方账号提供商',
    `openid`        varchar(128) CHARACTER SET utf8mb4 NOT NULL DEFAULT '' COMMENT '第三方账号唯一标识',
    --
    `access_token`  varchar(128) CHARACTER SET utf8mb4 NOT NULL DEFAULT '' COMMENT '第三方账号授权令牌',
    `refresh_token` varchar(128) CHARACTER SET utf8mb4 NOT NULL DEFAULT '' COMMENT '第三方账号刷新令牌',
    `expires_in`    int(11) unsigned NOT NULL DEFAULT '0' COMMENT '第三方账号授权令牌过期时间',
    `scope`         varchar(128) CHARACTER SET utf8mb4 NOT NULL DEFAULT '' COMMENT '第三方账号授权范围',
    `meta`          varchar(128) CHARACTER SET utf8mb4 NOT NULL DEFAULT '' COMMENT '第三方账号其他信息',
    -- common fields:
    `created_at`    TIMESTAMP                          NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at`    TIMESTAMP                          NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '修改时间',
    -- index fields:
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_provider_openid` (`provider`,`openid`), -- 索引: 第三方账号提供商/第三方账号唯一标识/唯一
    KEY             `idx_created_at` (`created_at`),       -- 建索引
    KEY             `idx_updated_at` (`updated_at`)        -- 建索引
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='用户第三方账号表';


-- 用户地址表:(收货地址) // 允许多个收货地址
CREATE TABLE `user_address`
(
    `id`         int(11) unsigned NOT NULL AUTO_INCREMENT COMMENT '自增主键(pk)',
    `user_id`    int(11) unsigned NOT NULL DEFAULT '0' COMMENT 'User ID',
    `order`      int(11) unsigned NOT NULL DEFAULT '0' COMMENT '地址优先级: 值越小越靠前 (默认0)',
    `label`      varchar(128) CHARACTER SET utf8mb4 NOT NULL DEFAULT '' COMMENT '地址标签',
    -- 收货人:
    `name`       varchar(128) CHARACTER SET utf8mb4 NOT NULL DEFAULT '' COMMENT '收货人姓名',
    `mobile`     varchar(128) CHARACTER SET utf8mb4 NOT NULL DEFAULT '' COMMENT '收货人手机号',
    `desc`       varchar(128) CHARACTER SET utf8mb4 NOT NULL DEFAULT '' COMMENT '收货地址描述',
    -- 地址信息:
    `country`    varchar(128) CHARACTER SET utf8mb4 NOT NULL DEFAULT '' COMMENT '国家',
    `province`   varchar(128) CHARACTER SET utf8mb4 NOT NULL DEFAULT '' COMMENT '省',
    `city`       varchar(128) CHARACTER SET utf8mb4 NOT NULL DEFAULT '' COMMENT '市',
    `district`   varchar(128) CHARACTER SET utf8mb4 NOT NULL DEFAULT '' COMMENT '区',
    `street`     varchar(128) CHARACTER SET utf8mb4 NOT NULL DEFAULT '' COMMENT '街道',
    `address`    varchar(128) CHARACTER SET utf8mb4 NOT NULL DEFAULT '' COMMENT '详细地址',
    `zipcode`    varchar(128) CHARACTER SET utf8mb4 NOT NULL DEFAULT '' COMMENT '邮编',

    -- common fields:
    `created_at` TIMESTAMP                          NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` TIMESTAMP                          NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '修改时间',
    -- index fields:
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_user_id_order` (`user_id`, `order`), -- 索引: 用户ID/优先级
    KEY          `idx_created_at` (`created_at`),       -- 建索引
    KEY          `idx_updated_at` (`updated_at`)        -- 建索引

) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='订单表';


-- ################################################################################################################


-- 商品表:
CREATE TABLE `product`
(
    `id`          int(11) unsigned NOT NULL AUTO_INCREMENT COMMENT '自增主键(pk)',
    `product_id`  int(11) unsigned NOT NULL DEFAULT '0' COMMENT '商品ID',
    `title`       varchar(128) CHARACTER SET utf8mb4 NOT NULL DEFAULT '' COMMENT '商品标题',
    `brief`       varchar(128) CHARACTER SET utf8mb4 NOT NULL DEFAULT '' COMMENT '商品简介',
    `description` varchar(255) CHARACTER SET utf8mb4 NOT NULL DEFAULT '' COMMENT '商品描述',
    `version`     varchar(255) CHARACTER SET utf8mb4 NOT NULL DEFAULT '' COMMENT '商品版本',
    `price`       decimal(10, 2) unsigned NOT NULL DEFAULT '0.00' COMMENT '商品价格',
    `status`      int(11) NOT NULL DEFAULT '0' COMMENT '状态: -1=下架, 0=未定义, 1=上架, 2=预发布',
    -- common fields:
    `created_at`  TIMESTAMP                          NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at`  TIMESTAMP                          NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '修改时间',
    -- index fields:
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_product_id` (`product_id`),
    KEY           `idx_created_at` (`created_at`), -- 建索引
    KEY           `idx_updated_at` (`updated_at`)  -- 建索引
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='商品表';


-- 商品分类表:
CREATE TABLE `product_category`
(
    `id`          int(11) unsigned NOT NULL AUTO_INCREMENT COMMENT '自增主键(pk)',
    `category_id` int(11) unsigned NOT NULL DEFAULT '0' COMMENT '分类ID',
    `parent_id`   int(11) unsigned NOT NULL DEFAULT '0' COMMENT '父级分类ID',
    --
    `level`       int(11) unsigned NOT NULL DEFAULT '0' COMMENT '分类级别',
    `order`       int(11) unsigned NOT NULL DEFAULT '0' COMMENT '排序',
    --
    `name`        varchar(128) CHARACTER SET utf8mb4 NOT NULL DEFAULT '' COMMENT '分类名称',
    `icon`        varchar(255) CHARACTER SET utf8mb4 NOT NULL DEFAULT '' COMMENT '分类图标',
    `description` varchar(255) CHARACTER SET utf8mb4 NOT NULL DEFAULT '' COMMENT '分类描述',
    `status`      int(11) NOT NULL DEFAULT '0' COMMENT '状态: -1=下架, 0=未定义, 1=上架, 2=预发布',
    -- common fields:
    `created_at`  TIMESTAMP                          NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at`  TIMESTAMP                          NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '修改时间',
    -- index fields:
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_category_id` (`category_id`),
    KEY           `idx_created_at` (`created_at`), -- 建索引
    KEY           `idx_updated_at` (`updated_at`)  -- 建索引
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='商品分类表';



-- ################################################################################################################

-- 商品属性表: 支持多语言(kv 表)
CREATE TABLE `product_attribute`
(
    `id`            int(11) unsigned NOT NULL AUTO_INCREMENT COMMENT '自增主键(pk)',
    `product_id`    int(11) unsigned NOT NULL DEFAULT '0' COMMENT '商品ID',
    `lang_id`       int(11) unsigned NOT NULL DEFAULT '1' COMMENT '语言ID: 0=默认语言, 1=中文, 2=英文',
    `attr_key`      varchar(128) CHARACTER SET utf8mb4 NOT NULL DEFAULT '' COMMENT '属性键',
    --
    `attr_key_name` varchar(128) CHARACTER SET utf8mb4 NOT NULL DEFAULT '' COMMENT '属性键名称',
    `attr_value`    varchar(128) CHARACTER SET utf8mb4 NOT NULL DEFAULT '' COMMENT '属性值',
    `status`        int(11) NOT NULL DEFAULT '0' COMMENT '状态: -1=下架, 0=未定义, 1=上架, 2=预发布',
    -- common fields:
    `created_at`    TIMESTAMP                          NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at`    TIMESTAMP                          NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '修改时间',
    -- index fields:
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_category_id_lang_id_attr_key` (`product_id`, `lang_id`, `attr_key`),
    KEY             `idx_created_at` (`created_at`), -- 建索引
    KEY             `idx_updated_at` (`updated_at`)  -- 建索引
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='商品属性表';


--  品牌表:
CREATE TABLE `brand`
(
    `id`          int(11) unsigned NOT NULL AUTO_INCREMENT COMMENT '自增主键(pk)',
    `brand_id`    int(11) unsigned NOT NULL DEFAULT '0' COMMENT '品牌ID',
    `name`        varchar(128) CHARACTER SET utf8mb4 NOT NULL DEFAULT '' COMMENT '品牌名称',
    `brief`       varchar(128) CHARACTER SET utf8mb4 NOT NULL DEFAULT '' COMMENT '品牌简介',
    `description` varchar(255) CHARACTER SET utf8mb4 NOT NULL DEFAULT '' COMMENT '品牌描述',
    `logo`        varchar(255) CHARACTER SET utf8mb4 NOT NULL DEFAULT '' COMMENT '品牌logo',
    `meta`        varchar(255) CHARACTER SET utf8mb4 NOT NULL DEFAULT '' COMMENT '品牌元数据',
    --
    `status`      int(11) NOT NULL DEFAULT '0' COMMENT '状态: -1=下架, 0=未定义, 1=上架, 2=预发布',
    `score`       decimal(10, 2) unsigned NOT NULL DEFAULT '0.00' COMMENT '品牌评分',
    `recommend`   int(11) NOT NULL DEFAULT '0' COMMENT '推荐指数: 0=不推荐, 1=推荐',
    -- common fields:
    `created_at`  TIMESTAMP                          NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at`  TIMESTAMP                          NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '修改时间',
    -- index fields:
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_brand_id` (`brand_id`),
    KEY           `idx_created_at` (`created_at`), -- 建索引
    KEY           `idx_updated_at` (`updated_at`)  -- 建索引
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='品牌表';


-- ################################################################################################################


--  购物车: (由缓存实现, 不使用 db 实现)
CREATE TABLE `cart`
(
    `id`               int(11) unsigned NOT NULL AUTO_INCREMENT COMMENT '自增主键(pk)',
    -- 购买者信息:
    `buyer_id`         int(11) unsigned NOT NULL DEFAULT '0' COMMENT '购买者ID',

    -- 店铺信息:
    `store_id`         int(11) unsigned NOT NULL DEFAULT '0' COMMENT '店铺ID',
    `store_name`       varchar(128) CHARACTER SET utf8mb4 NOT NULL DEFAULT '' COMMENT '店铺名称',

    -- 商品信息:
    `product_id`       int(11) unsigned NOT NULL DEFAULT '0' COMMENT '商品ID',
    `product_title`    varchar(128) CHARACTER SET utf8mb4 NOT NULL DEFAULT '' COMMENT '商品标题',
    `product_image`    varchar(255) CHARACTER SET utf8mb4 NOT NULL DEFAULT '' COMMENT '商品图片',
    `product_price`    decimal(10, 2) unsigned NOT NULL DEFAULT '0.00' COMMENT '商品价格',
    `product_amount`   int(11) unsigned NOT NULL DEFAULT '0' COMMENT '商品数量',
    `product_discount` decimal(10, 2) unsigned NOT NULL DEFAULT '0.00' COMMENT '商品折扣',
    `product_total`    decimal(10, 2) unsigned NOT NULL DEFAULT '0.00' COMMENT '商品总价',

    -- common fields:
    `created_at`       TIMESTAMP                          NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at`       TIMESTAMP                          NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '修改时间',
    -- index fields:
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_product_id` (`buyer_id`, `product_id`),
    KEY                `idx_created_at` (`created_at`), -- 建索引
    KEY                `idx_updated_at` (`updated_at`)  -- 建索引
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='购物车表';



-- ################################################################################################################


-- 用户订单表:
CREATE TABLE `order`
(
    `id`         int(11) unsigned NOT NULL AUTO_INCREMENT COMMENT '自增主键(pk)',
    `order_id`   int(11) unsigned NOT NULL DEFAULT '0' COMMENT '订单ID',
    -- buyer fields:
    `buyer_id`   int(11) unsigned NOT NULL DEFAULT '0' COMMENT '买家ID',
    `payment_id` int(11) unsigned NOT NULL DEFAULT '0' COMMENT '支付ID',
    `total`      decimal(10, 2) unsigned NOT NULL DEFAULT '0.00' COMMENT '总价',
    `status`     int(11) NOT NULL DEFAULT '0' COMMENT '状态: -1=超时关闭, 0=未定义, 1=已创建, 2=已完成',
    -- common fields:
    `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '修改时间',
    -- index fields:
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_order_id` (`order_id`),
    KEY          `idx_buyer_id` (`buyer_id`),     -- 建索引
    KEY          `idx_payment_id` (`payment_id`), -- 建索引
    KEY          `idx_created_at` (`created_at`), -- 建索引
    KEY          `idx_updated_at` (`updated_at`)  -- 建索引
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='用户订单表';


-- 用户订单明细表: (1:多) kv 表 // 买家 vs 卖家 // 支持单个买家 vs 多个卖家 (合并支付)
CREATE TABLE `order_item`
(
    `id`         int(11) unsigned NOT NULL AUTO_INCREMENT COMMENT '自增主键(pk)',
    `item_id`    int(11) unsigned NOT NULL DEFAULT '0' COMMENT '订单明细ID',
    -- related fields:
    `order_id`   int(11) unsigned NOT NULL DEFAULT '0' COMMENT '订单ID',
    `buyer_id`   int(11) unsigned NOT NULL DEFAULT '0' COMMENT '买家ID',
    `shop_id`    int(11) unsigned NOT NULL DEFAULT '0' COMMENT '店铺ID',
    `product_id` int(11) unsigned NOT NULL DEFAULT '0' COMMENT '商品ID',
    -- product fields:
    `quantity`   int(11) unsigned NOT NULL DEFAULT '0' COMMENT '商品数量',
    `price`      decimal(10, 2) unsigned NOT NULL DEFAULT '0.00' COMMENT '商品单价',
    `discount`   decimal(10, 2) unsigned NOT NULL DEFAULT '0.00' COMMENT '商品折扣',

    -- common fields:
    `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '修改时间',
    -- index fields:
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_item_id` (`item_id`),          -- 唯一索引: 订单项ID
    KEY          `idx_order_id` (`order_id`),     -- 建索引
    KEY          `idx_created_at` (`created_at`), -- 建索引
    KEY          `idx_updated_at` (`updated_at`)  -- 建索引
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='用户订单表';


-- 用户支付表:
CREATE TABLE `user_payment`
(
    `id`         int(11) unsigned NOT NULL AUTO_INCREMENT COMMENT '自增主键(pk)',
    `payment_id` int(11) unsigned NOT NULL DEFAULT '0' COMMENT '支付ID',
    `order_id`   int(11) unsigned NOT NULL DEFAULT '0' COMMENT '订单ID',
    `user_id`    int(11) unsigned NOT NULL DEFAULT '0' COMMENT '用户ID',
    -- payment fields:
    `pay_type`   int(11) unsigned NOT NULL DEFAULT '0' COMMENT '支付类型: 1=支付宝, 2=微信, 3=银联, 4=现金, 5=其他',
    `pay_sn`     varchar(64) NOT NULL DEFAULT '' COMMENT '支付单号',
    `total`      decimal(10, 2) unsigned NOT NULL DEFAULT '0.00' COMMENT '支付金额',
    -- common fields:
    `created_at` TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '修改时间',
    -- index fields:
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_payment_id` (`payment_id`),
    KEY          `idx_order_id` (`order_id`),     -- 建索引
    KEY          `idx_user_id` (`user_id`),       -- 建索引
    KEY          `idx_created_at` (`created_at`), -- 建索引
    KEY          `idx_updated_at` (`updated_at`)  -- 建索引
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='用户支付表';


-- 用户退款表:
CREATE TABLE `user_order_refund`
(
    `id`          int(11) unsigned NOT NULL AUTO_INCREMENT COMMENT '自增主键(pk)',
    `refund_id`   int(11) unsigned NOT NULL DEFAULT '0' COMMENT '退款ID',
    `order_id`    int(11) unsigned NOT NULL DEFAULT '0' COMMENT '订单ID',
    `user_id`     int(11) unsigned NOT NULL DEFAULT '0' COMMENT '用户ID',
    -- refund fields:
    `refund_type` int(11) unsigned NOT NULL DEFAULT '0' COMMENT '退款类型: 1=退款, 2=退货退款',
    `refund_sn`   varchar(64) NOT NULL DEFAULT '' COMMENT '退款单号',
    `total`       decimal(10, 2) unsigned NOT NULL DEFAULT '0.00' COMMENT '退款金额',
    -- common fields:
    `created_at`  TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at`  TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '修改时间',
    -- index fields:
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_refund_id` (`refund_id`),
    KEY           `idx_order_id` (`order_id`),     -- 建索引
    KEY           `idx_user_id` (`user_id`),       -- 建索引
    KEY           `idx_created_at` (`created_at`), -- 建索引
    KEY           `idx_updated_at` (`updated_at`)  -- 建索引
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='用户退款表';

-- ################################################################################################################
-- B 端:
-- 店铺表/ 店铺流水/ 店铺统计/ 店铺订单/ 店铺支付/ 店铺退款
-- ################################################################################################################


-- 店铺表:
CREATE TABLE `shop`
(
    `id`          int(11) unsigned NOT NULL AUTO_INCREMENT COMMENT '自增主键(pk)',
    `shop_id`     int(11) unsigned NOT NULL DEFAULT '0' COMMENT '店铺ID',
    `owner_id`    int(11) unsigned NOT NULL DEFAULT '0' COMMENT '店铺所有者ID',
    `name`        varchar(64)  NOT NULL DEFAULT '' COMMENT '店铺名称',
    `title`       varchar(128) NOT NULL DEFAULT '' COMMENT '店铺标题',
    `level`       int(11) unsigned NOT NULL DEFAULT '0' COMMENT '店铺等级',
    `description` varchar(255) NOT NULL DEFAULT '' COMMENT '店铺描述',
    -- 店铺状态:
    `is_verified` tinyint(1) unsigned NOT NULL DEFAULT '0' COMMENT '是否已认证: 0=未认证, 1=已认证',
    `is_official` tinyint(1) unsigned NOT NULL DEFAULT '0' COMMENT '是否官方店铺: 0=非官方店铺, 1=官方店铺',
    -- common fields:
    `status`      int(11) NOT NULL DEFAULT '0' COMMENT '状态: -1=封禁, 0=未定义, 1=正常',
    `created_at`  TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at`  TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '修改时间',
    -- index fields:
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_shop_id` (`shop_id`),
    KEY           `idx_owner_id` (`owner_id`),     -- 建索引
    KEY           `idx_created_at` (`created_at`), -- 建索引
    KEY           `idx_updated_at` (`updated_at`)  -- 建索引
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='店铺表';


-- 店铺设置: 财务账号表
CREATE TABLE `shop_setting_financial`
(
    `id`                       int(11) unsigned NOT NULL AUTO_INCREMENT COMMENT '自增主键(pk)',
    `shop_id`                  int(11) unsigned NOT NULL DEFAULT '0' COMMENT '店铺ID',
    `owner_id`                 int(11) unsigned NOT NULL DEFAULT '0' COMMENT '店铺所有者ID',
    --
    `bank_no`                  varchar(64) NOT NULL DEFAULT '' COMMENT '银行卡号',
    `bank_name`                varchar(64) NOT NULL DEFAULT '' COMMENT '开户行',
    `bank_branch`              varchar(64) NOT NULL DEFAULT '' COMMENT '开户支行',
    `bank_account_name`        varchar(64) NOT NULL DEFAULT '' COMMENT '开户名',
    `bank_account_address`     varchar(64) NOT NULL DEFAULT '' COMMENT '开户地址',
    `bank_account_phone`       varchar(64) NOT NULL DEFAULT '' COMMENT '开户电话',
    `bank_account_identity_no` varchar(64) NOT NULL DEFAULT '' COMMENT '开户身份证号',
    --
    `alipay_account`           varchar(64) NOT NULL DEFAULT '' COMMENT '支付宝账号',
    `alipay_name`              varchar(64) NOT NULL DEFAULT '' COMMENT '支付宝姓名',
    --
    `wechat_account`           varchar(64) NOT NULL DEFAULT '' COMMENT '微信账号',
    `wechat_name`              varchar(64) NOT NULL DEFAULT '' COMMENT '微信姓名',
    --
    `paypal_account`           varchar(64) NOT NULL DEFAULT '' COMMENT 'PayPal账号',
    `paypal_name`              varchar(64) NOT NULL DEFAULT '' COMMENT 'PayPal姓名',
    -- 数字货币:
    `eth_address`              varchar(64) NOT NULL DEFAULT '' COMMENT 'ETH地址',
    `usdt_address`             varchar(64) NOT NULL DEFAULT '' COMMENT 'USDT地址',
    `btc_address`              varchar(64) NOT NULL DEFAULT '' COMMENT 'BTC地址',
    `doge_address`             varchar(64) NOT NULL DEFAULT '' COMMENT 'DOGE地址',
    -- common fields:
    `status`                   int(11) NOT NULL DEFAULT '0' COMMENT '状态: -1=封禁, 0=未定义, 1=正常',
    `created_at`               TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at`               TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '修改时间',
    -- index fields:
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_shop_id` (`shop_id`),
    KEY                        `idx_owner_id` (`owner_id`),     -- 建索引
    KEY                        `idx_created_at` (`created_at`), -- 建索引
    KEY                        `idx_updated_at` (`updated_at`)  -- 建索引
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='店铺表';


-- 店铺设置: 物流设置表(方案)
CREATE TABLE `shop_setting_delivery`
(
    `id`                       int(11) unsigned NOT NULL AUTO_INCREMENT COMMENT '自增主键(pk)',
    `shop_id`                  int(11) unsigned NOT NULL DEFAULT '0' COMMENT '店铺ID',
    --
    `delivery_id`              int(11) unsigned NOT NULL DEFAULT '0' COMMENT '物流方案ID',
    `name`                     varchar(64)  NOT NULL DEFAULT '' COMMENT '方案名称',
    `description`              varchar(255) NOT NULL DEFAULT '' COMMENT '方案描述',
    `is_default`               tinyint(1) NOT NULL DEFAULT '0' COMMENT '是否默认方案',
    -- 类型:
    `delivery_type`            int(11) unsigned NOT NULL DEFAULT '0' COMMENT '配送方式: 0=未定义, 1=快递, 2=EMS, 3=平邮, 4=自提',
    `delivery_scope`           int(11) unsigned NOT NULL DEFAULT '0' COMMENT '配送范围: 0=未定义, 1=全国, 2=省内, 3=市内, 4=区内',
    -- 具体设置:
    `delivery_fee`             decimal(10, 2) unsigned NOT NULL DEFAULT '0.00' COMMENT '配送费用',
    `delivery_free_fee`        decimal(10, 2) unsigned NOT NULL DEFAULT '0.00' COMMENT '配送免费金额',
    `delivery_free_fee_type`   int(11) unsigned NOT NULL DEFAULT '0' COMMENT '配送免费金额类型: 0=未定义, 1=按金额, 2=按件数',
    `delivery_free_fee_count`  int(11) unsigned NOT NULL DEFAULT '0' COMMENT '配送免费金额按件数',
    `delivery_free_fee_weight` decimal(10, 2) unsigned NOT NULL DEFAULT '0.00' COMMENT '配送免费金额按重量',
    `delivery_free_fee_volume` decimal(10, 2) unsigned NOT NULL DEFAULT '0.00' COMMENT '配送免费金额按体积',
    -- common fields:
    `status`                   int(11) NOT NULL DEFAULT '0' COMMENT '状态: -1=封禁, 0=未定义, 1=正常',
    `created_at`               TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at`               TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '修改时间',
    -- index fields:
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_delivery_id` (`delivery_id`),
    KEY                        `idx_shop_id` (`shop_id`),       -- 建索引
    KEY                        `idx_created_at` (`created_at`), -- 建索引
    KEY                        `idx_updated_at` (`updated_at`)  -- 建索引
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='店铺表';


-- 店铺认证信息表:
CREATE TABLE `shop_verify`
(
    `id`                       int(11) unsigned NOT NULL AUTO_INCREMENT COMMENT '自增主键(pk)',
    `shop_id`                  int(11) unsigned NOT NULL DEFAULT '0' COMMENT '店铺ID',
    `owner_id`                 int(11) unsigned NOT NULL DEFAULT '0' COMMENT '店铺所有者ID',
    `verify_type`              int(11) unsigned NOT NULL DEFAULT '0' COMMENT '认证类型: 1=身份证, 2=营业执照',

    -- 企业认证信息:
    `owner_corp`               varchar(64)  NOT NULL DEFAULT '' COMMENT '店铺所有者公司名称',
    `owner_corp_code`          varchar(64)  NOT NULL DEFAULT '' COMMENT '店铺所有者公司统一社会信用代码',
    `owner_corp_file`          varchar(255) NOT NULL DEFAULT '' COMMENT '店铺营业执照: 图片',
    `owner_corp_address`       varchar(64)  NOT NULL DEFAULT '' COMMENT '店铺所有者公司地址',
    `owner_corp_tel`           varchar(64)  NOT NULL DEFAULT '' COMMENT '店铺所有者公司电话',
    `owner_corp_fax`           varchar(64)  NOT NULL DEFAULT '' COMMENT '店铺所有者公司传真',

    -- 个人认证信息:
    `owner_name`               varchar(64)  NOT NULL DEFAULT '' COMMENT '店铺所有者姓名',
    `owner_phone`              varchar(64)  NOT NULL DEFAULT '' COMMENT '店铺所有者手机号',
    `owner_email`              varchar(128) NOT NULL DEFAULT '' COMMENT '店铺所有者邮箱',
    `owner_id_card`            varchar(64)  NOT NULL DEFAULT '' COMMENT '店铺所有者身份证号',
    `owner_id_card_front`      varchar(255) NOT NULL DEFAULT '' COMMENT '店铺所有者身份证正面照',
    `owner_id_card_back`       varchar(255) NOT NULL DEFAULT '' COMMENT '店铺所有者身份证背面照',
    `owner_id_card_hand_front` varchar(255) NOT NULL DEFAULT '' COMMENT '店铺所有者手持身份证正面照',
    `owner_id_card_hand_back`  varchar(255) NOT NULL DEFAULT '' COMMENT '店铺所有者手持身份证背面照',

    -- 认证状态:
    `verify_status`            int(11) unsigned NOT NULL DEFAULT '0' COMMENT '认证状态: 0=未认证, 1=个人认证, 2=企业认证, 3=个人+企业认证',

    -- common fields:
    `status`                   int(11) NOT NULL DEFAULT '0' COMMENT '状态: -1=封禁, 0=未定义, 1=正常',
    `created_at`               TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at`               TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '修改时间',
    -- index fields:
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_shop_id` (`shop_id`),
    KEY                        `idx_owner_id` (`owner_id`),     -- 建索引
    KEY                        `idx_created_at` (`created_at`), -- 建索引
    KEY                        `idx_updated_at` (`updated_at`)  -- 建索引
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='店铺表';


-- 店铺员工表: (1:多)
CREATE TABLE `shop_employee`
(
    `id`         int(11) unsigned NOT NULL AUTO_INCREMENT COMMENT '自增主键(pk)',
    `shop_id`    int(11) unsigned NOT NULL DEFAULT '0' COMMENT '店铺ID',
    `user_id`    int(11) unsigned NOT NULL DEFAULT '0' COMMENT '用户ID',
    `name`       varchar(64)  NOT NULL DEFAULT '' COMMENT '员工姓名',
    `remark`     varchar(255) NOT NULL DEFAULT '' COMMENT '备注',
    `role`       int(11) unsigned NOT NULL DEFAULT '0' COMMENT '员工角色: 1=店长, 2=店员',
    -- common fields:
    `status`     int(11) NOT NULL DEFAULT '0' COMMENT '状态: -1=封禁, 0=未定义, 1=正常',
    `created_at` TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '修改时间',
    -- index fields:
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_shop_id_user_id` (`shop_id`, `user_id`),
    KEY          `idx_created_at` (`created_at`), -- 建索引
    KEY          `idx_updated_at` (`updated_at`)  -- 建索引
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='店铺员工表';


-- 店铺上架商品表: (1:多)
CREATE TABLE `shop_product`
(
    `id`         int(11) unsigned NOT NULL AUTO_INCREMENT COMMENT '自增主键(pk)',
    `shop_id`    int(11) unsigned NOT NULL DEFAULT '0' COMMENT '店铺ID',
    --
    `product_id` int(11) unsigned NOT NULL DEFAULT '0' COMMENT '商品ID',
    `stock`      int(11) unsigned NOT NULL DEFAULT '0' COMMENT '库存容量',
    --
    `ver`        int(11) NOT NULL DEFAULT '1' COMMENT '版本号，用于乐观锁',
    -- common fields:
    `status`     int(11) NOT NULL DEFAULT '0' COMMENT '状态: -1=封禁, 0=未定义, 1=正常, 2=预售',
    `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '修改时间',
    -- index fields:
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_shop_id_product_id` (`shop_id`, `product_id`), -- 建索引: 店铺+商品/唯一
    KEY          `idx_created_at` (`created_at`),                 -- 建索引
    KEY          `idx_updated_at` (`updated_at`)                  -- 建索引
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='店铺员工表';


-- 店铺订单表: (1:多)
CREATE TABLE `shop_order`
(
    `id`            int(11) unsigned NOT NULL AUTO_INCREMENT COMMENT '自增主键(pk)',
    `shop_id`       int(11) unsigned NOT NULL DEFAULT '0' COMMENT '店铺ID',
    `order_id`      int(11) unsigned NOT NULL DEFAULT '0' COMMENT '订单ID',
    `order_item_id` int(11) unsigned NOT NULL DEFAULT '0' COMMENT '订单项ID',
    `income`        decimal(10, 2) NOT NULL DEFAULT '0.00' COMMENT '订单收入',
    --
    `ver`           int(11) NOT NULL DEFAULT '1' COMMENT '版本号，用于乐观锁',
    -- common fields:
    `status`        int(11) NOT NULL DEFAULT '0' COMMENT '状态: -1=封禁, 0=未定义, 1=正常, 2=预售',
    `created_at`    TIMESTAMP      NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at`    TIMESTAMP      NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '修改时间',
    -- index fields:
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_order_item_id` (`order_item_id`), -- 建索引: 店铺+商品/唯一
    KEY             `idx_shop_id` (`shop_id`),       -- 建索引
    KEY             `idx_order_id` (`order_id`),     -- 建索引
    KEY             `idx_created_at` (`created_at`), -- 建索引
    KEY             `idx_updated_at` (`updated_at`)  -- 建索引
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='店铺员工表';


-- 店铺日结流水表: (1:多)
CREATE TABLE `shop_bill_daily`
(
    `id`         int(11) unsigned NOT NULL AUTO_INCREMENT COMMENT '自增主键(pk)',
    `shop_id`    int(11) unsigned NOT NULL DEFAULT '0' COMMENT '店铺ID',
    `income`     decimal(10, 2) NOT NULL DEFAULT '0.00' COMMENT '订单收入',
    `date`       date           NOT NULL DEFAULT (CURRENT_DATE) COMMENT '日期',
    --
    `ver`        int(11) NOT NULL DEFAULT '1' COMMENT '版本号，用于乐观锁',
    -- common fields:
    `created_at` TIMESTAMP      NOT NULL DEFAULT (CURRENT_TIMESTAMP) COMMENT '创建时间',
    `updated_at` TIMESTAMP      NOT NULL DEFAULT (CURRENT_TIMESTAMP) ON UPDATE CURRENT_TIMESTAMP COMMENT '修改时间',
    -- index fields:
    PRIMARY KEY (`id`),
    KEY          `idx_shop_id` (`shop_id`),       -- 建索引
    KEY          `idx_date` (`date`),             -- 建索引
    KEY          `idx_created_at` (`created_at`), -- 建索引
    KEY          `idx_updated_at` (`updated_at`)  -- 建索引
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='店铺员工表';


-- ################################################################################################################
-- 评论系统: 商品评价/店铺评价/社区讨论/工单/投诉/建议
-- ################################################################################################################


-- 评价表: (1:多)
CREATE TABLE `comment`
(
    `id`         int(11) unsigned NOT NULL AUTO_INCREMENT COMMENT '自增主键(pk)',
    `comment_id` int(11) unsigned NOT NULL DEFAULT '0' COMMENT '评价ID',
    `user_id`    int(11) unsigned NOT NULL DEFAULT '0' COMMENT '用户ID',
    `relate_id`  int(11) unsigned NOT NULL DEFAULT '0' COMMENT '关联ID: 商品ID/店铺ID/社区ID/工单ID/投诉ID/建议ID/评论 ID',
    `type`       int(11) unsigned NOT NULL DEFAULT '0' COMMENT '类型: 0=商品评价, 1=店铺评价, 2=社区讨论, 3=工单, 4=投诉, 5=建议',
    -- 内容:
    `content`    varchar(255) CHARACTER SET utf8mb4 NOT NULL DEFAULT '' COMMENT '评价内容',
    `score`      int(11) unsigned NOT NULL DEFAULT '0' COMMENT '评分: 1=差评, 2=中评, 3=好评',
    -- common fields:
    `status`     int(11) NOT NULL DEFAULT '1' COMMENT '状态: -1=封禁, 0=未定义, 1=正常, 2=预售',
    `created_at` TIMESTAMP                          NOT NULL DEFAULT (CURRENT_TIMESTAMP) COMMENT '创建时间',
    `updated_at` TIMESTAMP                          NOT NULL DEFAULT (CURRENT_TIMESTAMP) ON UPDATE CURRENT_TIMESTAMP COMMENT '修改时间',
    -- index fields:
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_comment_id` (`comment_id`),    -- 建索引: 唯一
    KEY          `idx_user_id` (`user_id`),       -- 建索引
    KEY          `idx_relate_id` (`relate_id`),   -- 建索引
    KEY          `idx_created_at` (`created_at`), -- 建索引
    KEY          `idx_updated_at` (`updated_at`)  -- 建索引
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='评论系统表';



-- ################################################################################################################


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
