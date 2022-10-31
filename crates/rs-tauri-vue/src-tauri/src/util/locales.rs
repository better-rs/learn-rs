use sys_locale::get_locale;

pub fn sys_locale() -> String {
    // 转换系统语言: 匹配多语言配置文件
    match get_locale().unwrap_or(String::from("en")).as_str() {
        // "zh-Hans-CN" => String::from("zh-CN"),
        // "zh-Hans-HK" => String::from("zh-HK"),
        v => {
            if v.matches("-").count() == 2 {
                let s: Vec<&str> = v.split("-").collect();
                return format!("{}-{}", s[0], s[2])
            }
            String::from(v)
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::util::sys_locale;

    #[test]
    fn it_works() {
        let l = sys_locale();
        println!("The current locale is {}", l);
    }
}
