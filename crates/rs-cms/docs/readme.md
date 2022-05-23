# rs-cms:

- 基于 rust 实现的 cms.

## 架构说明:

- 用户系统设计(C 端)
    - 用户注册、登录、登出
    - 用户角色、权限
    - 用户组
    - 用户认证
    - 用户状态
    - 用户支付
    - 用户订单管理
    - 用户账单管理
- 店铺系统设计(B 端)
    - 店铺管理
    - 店铺订单
    - 店铺商品
    - 店铺营销
    - 店铺支付
    - 店铺统计
    - 店铺推广
    - 店铺财务
    - 店铺物流管理
    - 店铺组织管理
    - 店铺库存管理
- 商品系统设计
    - 商品管理
        - 商品定义
        - SKU
    - 商品分类
    - 商品规格
    - 商品属性
- 购物车系统
    - 添加/删除/更新购物车
- 秒杀系统
    - 开启/关闭秒杀
    - 秒杀商品
    - 秒杀订单
- 订单系统设计
    - 订单管理
    - 订单支付
    - 订单发货
    - 订单退款/退货
- IM 系统:
    - 客服沟通
    - 申诉/投诉/仲裁
- 内容评价系统设计
    - 商品评价
    - 店铺评价
    - 订单评价
    - 商品咨询
- 平台管理系统设计
    - 审核平台
        - 商品/店铺上下架审核
    - 工单系统
    - 客户投诉管理平台
    - 广告管理平台

## db model 设计:

- user: 用户表

```ruby 

id :
user_id:
username: 用户名
email: 注册邮箱
password: 密码
is_verified: 邮箱验证状态: 0 未验证, 1 验证通过
is_active: 账号状态: 0: 未激活, 1: 激活, 2: 冻结
roles: 角色: admin, staff, user
created_at:
updated_at:

```

- user_profile: 用户资料表

```ruby         



```

- user_pay_info: 用户支付信息表

```ruby         

id:
user_id:
pay_type: 支付方式: alipay, wechat, bank, btc, eth, etc.
pay_account: 支付账号
pay_password: 支付密码
pay_status: 支付状态



```

- payment:

```ruby 

id:
user_id:
amount: 支付
payment_method: alipay, wechat, bank, cash, btc, eth, usdt, doge,  // 支付方式
created_at:
updated_at:



```

- product: 产品 SKU 定义

```ruby     

id:
product_id: 产品编号
product_name: 产品名称
product_desc: 产品描述
product_price: 产品价格
product_status: 产品状态: 0: 下架, 1: 上架
product_type: 产品类型: 0: 普通产品, 1: 充值产品
product_image: 产品图片
product_image_url: 产品图片 url
product_url: 产品链接
product_version: 产品版本
created_at:
updated_at:

```

- order:

```ruby 
id:
order_id:
user_id:
product_id:


```

- license:

```ruby 

id:
user_id:
license_id:
status:

```

- user_product_license: 预先随机生成一批 license_id 和 product_id 的关系

```ruby 

id: 
user_id: 用户名
product_id: 产品编号
license_id: 注册码


key:
status:


```

- license_product_key: 密钥

```ruby 


```

## ref:

- https://github.com/one-ai/one-license-server

> sql:

- https://github.com/macrozheng/mall/blob/master/document/sql/mall.sql

> mall 设计参考:

- http://static.kancloud.cn/yejingjing/hcclub_v7/1241902
    - 购物车: http://static.kancloud.cn/yejingjing/hcclub_v7/1241916
    - 订单流程: http://static.kancloud.cn/yejingjing/hcclub_v7/1241917