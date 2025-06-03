use std::{
    error::Error,
    fmt::Display,
    fs::{self, File},
    io::{Read, Write},
    path::Path,
};

use lightningcss::{
    properties::{
        Property,
        custom::{CustomPropertyName, Token, TokenOrValue},
    },
    rules::CssRule,
    stylesheet::{ParserOptions, PrinterOptions, StyleSheet},
    targets::{Browsers, Targets},
    values::length::LengthValue,
    visit_types,
    visitor::{Visit, VisitTypes, Visitor},
};

use std::convert::Infallible;
#[derive(Default, Debug, Clone, Copy)]
pub struct PaOptions<'a> {
    pub font_num: Option<f32>,
    pub be_width: Option<f32>,
    pub be_height: Option<f32>,
    pub checktype: Option<&'a str>,
    pub file_path: Option<&'a str>,
}
#[derive(Debug)]
struct MyVisitor<'a> {
    pa_option: PaOptions<'a>,
}

impl<'a, 'i> Visitor<'i> for MyVisitor<'a> {
    type Error = Infallible;
    fn visit_types(&self) -> VisitTypes {
        // visit_types!(URLS | LENGTHS | RULES) // 添加 RULES 以访问规则
        //VARIABLES FUNCTIONS与TOKENS相冲突 一起用会失效
        visit_types!(LENGTHS | MEDIA_QUERIES | TOKENS)
    }

    // fn visit_media_feature_value(
    //     &mut self,
    //     value: &mut MediaFeatureValue<'i>,
    // ) -> Result<(), Self::Error> {
    //      //可用来跳过其他@媒体查询的条件部分 如@container
    //     // if let MediaFeatureValue::Length(length) = value {
    //     //     match length {
    //     //         Value(px) => {
    //     //             println!("mmmmmm med{:?}", px);
    //     //         }
    //     //         _ => {}
    //     //     }
    //     // }
    //     Ok(())
    // }
    // // 跳过媒体查询的条件部分
    fn visit_media_query(
        &mut self,
        _query: &mut lightningcss::media_query::MediaQuery<'i>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
    // fn visit_property(&mut self, property: &mut lightningcss::properties::Property<'i>) -> Result<(), Self::Error> {
    //  println!("pro__{:?}",property);
    //     Ok(())
    // }

    // fn visit_rule(&mut self, rule: &mut CssRule<'i>) -> Result<(), Self::Error> {
    //     match rule {
    //         CssRule::Media(MediaRule { query, rules, .. }) => {
    //             // 进入媒体查询的条件部分
    //             self.in_media_condition = true;
    //             // 访问媒体查询的条件部分（例如 max-width: 500px）
    //             query.visit(self)?;
    //             self.in_media_condition = false;
    //             // // 访问媒体查询内部的规则（例如 .foo { width: 100px }）
    //             for rule in rules.0.iter_mut() {
    //                 // 使用 iter_mut() 获取可变迭代器
    //                 self.visit_rule(rule)?;
    //             }
    //         }
    //         _ => {
    //             // 访问其他规则
    //             rule.visit_children(self)?;
    //         }
    //     }

    //     match rule {
    //         CssRule::Style(style_rule) => {
    //             // println!("declaration_{:?}", style_rule.declarations.declarations[0]);
    //             match &style_rule.declarations.declarations[0] {
    //                 Property::Custom(custom_property) => {
    //                     if let CustomPropertyName::Unknown(ident) = &custom_property.name {
    //                         println!("runame__{:?}", ident.0);
    //                     }
    //                 }
    //                 _ => {}
    //             }
    //         }

    //         _ => {
    //             // 访问其他规则
    //             rule.visit_children(self)?;
    //         }
    //     }

    //     Ok(())
    // }
    // fn visit_property(&mut self, property: &mut Property<'i>) -> Result<(), Self::Error> {
    //     println!("property_{:?}", property);
    //     if let Property::Custom(custom_property) = property {
    //         if let CustomPropertyName::Unknown(ident) = &custom_property.name {
    //             println!("name__{:?}", ident.0);
    //         }
    //     }
    //     Ok(())
    // }

    fn visit_token(&mut self, token: &mut TokenOrValue<'i>) -> Result<(), Self::Error> {
        // println!("token_{:?}", token);
        match token {
            TokenOrValue::Length(length) => {
                if let LengthValue::Px(px) = length {
                    *length = conditional_px_conversion(*px, self.pa_option);
                }
            }
            TokenOrValue::Function(f)
                if self.pa_option.checktype.unwrap_or("") == "1" && f.name.0 == "vh" =>
            {
                if let Some(arg) = f.arguments.0.get(0) {
                    if let TokenOrValue::Token(num) = arg {
                        if let Token::Number { value, .. } = num {
                            let be_height = self.pa_option.be_height.unwrap_or(1080.0);
                            let abs_value = value.abs();
                            let length = if (abs_value - 1.0).abs() > f32::EPSILON {
                                let vh_percent =
                                    (value / be_height * 100.0 * 1000.0).round() / 1000.0;
                                LengthValue::Vh(vh_percent)
                            } else {
                                LengthValue::Px(*value)
                            };
                            *token = TokenOrValue::Length(length);
                        }
                    }

                    // if let TokenOrValue::Token(Token::Number { value, .. }) = arg {
                    //     println!("value is {}", value);
                    // }
                }
            }
            TokenOrValue::Token(token) => {
                // println!("ttt_{:?}",token);
                // let s = "\u{e6a0}";
                // println!("{}", s); // 打印: 
                // println!("{:?}", s);
                if let Token::String(str) = token {
                    println!("str____{:?}", str);
                    // let escaped = escape_unicode_chars(str);
                    // println!("escaped____{:?}", escaped);
                    // *str = escaped.into();

                }
            }
            _ => {}
        }

        Ok(())
    }

    fn visit_length(&mut self, length: &mut LengthValue) -> Result<(), Self::Error> {
        let pa_option = self.pa_option;
        if let LengthValue::Px(px) = length {
            *length = conditional_px_conversion(*px, pa_option);
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub enum CSSError {
    NoCssFilesFound,
    CSSSyntaxError,
    OtherError,
}
impl Display for CSSError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CSSError::NoCssFilesFound => {
                write!(f, "No CSS files found in the directory")
            }
            CSSError::CSSSyntaxError => write!(f, "CSS解析错误"),
            CSSError::OtherError => write!(f, "其他错误"),
            // CSSError::OtherError(msg) => write!(f, "其他错误: {}", msg),
        }
    }
}
impl Error for CSSError {}

pub fn read_file(pa_option: PaOptions) -> Result<(), Box<dyn Error>> {
    let file_path = pa_option.file_path.unwrap_or("");
    let mut css_files_processed = false;
    // println!("{}", "start");
    for file_item in fs::read_dir(&file_path)? {
        // println!("file_item_{:?}", file_item);
        let file_item = file_item?.path();
        if file_item.is_dir() {
            // 递归处理子目录
            let mut sub_options = pa_option.clone();
            let file_path = file_item.to_str().unwrap().to_string();
            sub_options.file_path = Some(&file_path);
            // 递归调用read_file并合并结果
            match read_file(sub_options) {
                Ok(_) => css_files_processed = true,
                Err(e) => {
                    println!("err____{:?}", e);
                    // 如果子目录中没有CSS文件，继续处理其他项
                    if let Some(err) = e.downcast_ref::<CSSError>() {
                        if *err == CSSError::NoCssFilesFound {
                            continue;
                        }
                    }
                    return Err(e);
                }
            }
        } else if file_item.is_file() {
            if let Some(ext) = file_item.extension() {
                if ext.to_str() == Some("css") {
                    // let file_name = file_item.file_stem().unwrap().to_str().unwrap();
                    let file_name = file_item
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or_default();
                    if !file_name.ends_with("_conv_rem") && !file_name.ends_with("_conv_vw") {
                        let mut file = File::open(&file_item)?;
                        let mut content = String::new();
                        file.read_to_string(&mut content)?;
                        let content = unit_analysis_change(pa_option, &content)?;
                        let css_unit = match pa_option.checktype.unwrap_or("_") {
                            "0" => "rem",
                            "1" => "vw",
                            _ => "_",
                        };
                        let new_name = format!("{}_conv_{}.css", file_name, css_unit);
                        let new_file_path = Path::new(file_path).join(&new_name);
                        // println!("new_file_path_{:?}", new_file_path);
                        let mut new_file = File::create(new_file_path)?;
                        new_file.write_all(content.as_bytes())?;
                        css_files_processed = true;
                    }
                }
            }
        }
    }
    // println!("{}", "end");
    if !css_files_processed {
        return Err(Box::new(CSSError::NoCssFilesFound));
    }
    Ok(())
}

fn unit_analysis_change(pa_option: PaOptions, css: &str) -> Result<String, Box<dyn Error>> {
    // 创建一个用于存储替换后的 CSS 代码的字符串
    let mut replaced_css = String::new();
    match StyleSheet::parse(css, ParserOptions::default()) {
        Ok(mut stylesheet) => {
            let mut visitor = MyVisitor { pa_option };
            stylesheet.visit(&mut visitor).unwrap();
            let targets = Targets {
                browsers: Some(Browsers {
                    chrome: Some(50), // 设置转义后所支持的浏览器版本，
                    // ie: Some(9),
                    ..Default::default()
                }),
                ..Default::default()
            };

            let res = stylesheet
                .to_css(PrinterOptions {
                    // minify: true,
                    targets,
                    ..Default::default()
                })
                .unwrap();
            replaced_css.push_str(&res.code);
            Ok(replaced_css)
        }
        // Err(e) => match e.kind {
        //     ParserError::UnexpectedToken(_e) => Err(Box::new(CSSError::CSSSyntaxError)),
        //     _ => {
        //         println!("err___{:?}",e);
        //         Err(Box::new(CSSError::OtherError))
        //     }
        //    ,
        // },
        Err(_e) => Err(Box::new(CSSError::CSSSyntaxError)),
    }
}

fn conditional_px_conversion(px: f32, pa_option: PaOptions) -> LengthValue {
    let checktype = pa_option.checktype.unwrap_or("");
    let px_val = px.abs();
    if (px_val - 1.0).abs() > f32::EPSILON {
        match checktype {
            "0" => {
                let font_value = pa_option.font_num.unwrap_or(50.0);
                LengthValue::Rem(px / font_value)
            }
            "1" => {
                let be_width = pa_option.be_width.unwrap_or(1920.0);
                let result = (px / be_width * 100.0 * 1000.0).round() / 1000.0;
                LengthValue::Vw(result)
            }
            _ => LengthValue::Px(px),
        }
    } else {
        LengthValue::Px(px)
    }
}
fn escape_unicode_chars(s: &str) -> String {
    let mut result = String::new();
    for c in s.chars() {
        if c.is_ascii() {
            result.push(c);
        } else {
            let code_point = c as u32;
            result.push_str(&format!("\\{:x}", code_point));
        }
    }
    result
}
 