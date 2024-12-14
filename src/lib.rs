use nvim_oxi as oxi;
use oxi::{api, Dictionary, Function, Object};
use regex::Regex;

#[oxi::plugin]
fn nvim_unicode_converter() -> Dictionary {
    let convert_unicode = Function::from_fn(|()| {
        // 获取当前选中的文本
        let current_selection = api::get_current_line().unwrap();

        // 正则表达式匹配 \uXXXX 格式
        let re = Regex::new(r"\\u([0-9a-fA-F]{4})").unwrap();

        // 替换匹配的部分
        let converted = re.replace_all(&current_selection, |caps: &regex::Captures| {
            let code_point = u32::from_str_radix(&caps[1], 16).unwrap();
            std::char::from_u32(code_point).unwrap().to_string()
        });

        // 将结果替换到当前行
        let _ = api::set_current_line(converted);
    });

    Dictionary::from_iter([("convert_unicode", Object::from(convert_unicode))])
}
