# rs-cms:

- 基于 rust 实现的 cms.

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