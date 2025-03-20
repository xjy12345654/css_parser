use std::{
    error::Error,
    fs::{self, File},
    io::{Read, Write},
};

use lightningcss::{
    properties::custom::{Token, TokenOrValue},
    rules::{CssRule, media::MediaRule},
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
}
#[derive(Debug)]
struct MyVisitor<'a> {
    in_media_condition: bool, // 用于跟踪是否在媒体查询的条件部分
    pa_option: PaOptions<'a>,
}

impl<'a, 'i> Visitor<'i> for MyVisitor<'a> {
    type Error = Infallible;
    fn visit_types(&self) -> VisitTypes {
        // visit_types!(URLS | LENGTHS | RULES) // 添加 RULES 以访问规则
        visit_types!(TOKENS | RULES | LENGTHS) // 添加 RULES 以访问规则
    }

    // fn visit_url(&mut self, url: &mut Url<'i>) -> Result<(), Self::Error> {
    //     url.url = format!("https://test/{}", url.url).into();
    //     Ok(())
    // }

    fn visit_rule(&mut self, rule: &mut CssRule<'i>) -> Result<(), Self::Error> {
        match rule {
            CssRule::Media(MediaRule { query, rules, .. }) => {
                // 进入媒体查询的条件部分
                self.in_media_condition = true;
                // 访问媒体查询的条件部分（例如 max-width: 500px）
                query.visit(self)?;
                self.in_media_condition = false;
                // // 访问媒体查询内部的规则（例如 .foo { width: 100px }）
                for rule in rules.0.iter_mut() {
                    // 使用 iter_mut() 获取可变迭代器
                    self.visit_rule(rule)?;
                }
            }
            _ => {
                // 访问其他规则
                rule.visit_children(self)?;
            }
        }

        Ok(())
    }

    fn visit_token(&mut self, token: &mut TokenOrValue<'i>) -> Result<(), Self::Error> {
        // println!("{:?}", token);
        if self.pa_option.checktype.unwrap() == "1" {
            match token {
                TokenOrValue::Function(f) if f.name.0 == "vh" => {
                    // println!("{:?}", f.arguments.0.get(0));
                    if let Some(arg) = f.arguments.0.get(0) {
                        if let TokenOrValue::Token(num) = arg {
                            if let Token::Number { value, .. } = num {
                                // let value = *value;
                                // if value.abs() != 1.0 {
                                //     let be_height = self.pa_option.be_height.unwrap_or(1080.0);
                                //     let result =
                                //         (value / be_height * 100.0 * 1000.0).round() / 1000.0;
                                //     // 创建一个新的长度值
                                //     let length = LengthValue::Vh(result);
                                //     // 替换为长度值
                                //     *token = TokenOrValue::Length(length);
                                // } else {
                                //     let length = LengthValue::Px(value);
                                //     *token = TokenOrValue::Length(length);
                                // }

                                let abs_value = value.abs();
                                let is_unit_value = (abs_value - 1.0).abs() <= f32::EPSILON;
                                let be_height = self.pa_option.be_height.unwrap_or(1080.0);

                                let length = if !is_unit_value {
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
                _ => {}
            }
        }

        Ok(())
    }

    fn visit_length(&mut self, length: &mut LengthValue) -> Result<(), Self::Error> {
        // 如果不在媒体查询的条件部分，则进行转换
        let pa_option = self.pa_option;
        let checktype = pa_option.checktype.unwrap();

        if !self.in_media_condition {
            match checktype {
                "0" => {
                    let font_value = pa_option.font_num.unwrap_or(50.0);
                    // match length {
                    //     LengthValue::Px(px) => {
                    //         let px = *px;
                    //         if px.abs() != 1.0 {
                    //             *length = LengthValue::Rem(px / font_value);
                    //         }
                    //     }
                    //     _ => {}
                    // }

                    if let LengthValue::Px(px) = length {
                        // let px_val = px.abs();
                        // let px = *px;
                        // if px.abs() != 1.0 {
                        //     *length = LengthValue::Rem(px / font_value);
                        // }

                        let px_val = px.abs();
                        // println!("{}", (px_val - 1.0).abs() > f32::EPSILON);
                        if (px_val - 1.0).abs() > f32::EPSILON {
                            *length = LengthValue::Rem(*px / font_value);
                        }
                    }
                }
                "1" => {
                    let be_width = pa_option.be_width.unwrap_or(1920.0);

                    // match length {
                    //     LengthValue::Px(px) => {
                    //         let px = *px;
                    //         if px.abs() != 1.0 {
                    //             let result = (px / be_width * 100.0 * 1000.0).round() / 1000.0;
                    //             *length = LengthValue::Vw(result);
                    //         }
                    //     }
                    //     _ => {}
                    // }

                    if let LengthValue::Px(px) = length {
                        // let px = *px;
                        // if px.abs() != 1.0 {
                        //     let result = (px / be_width * 100.0 * 1000.0).round() / 1000.0;
                        //     *length = LengthValue::Vw(result);
                        // }
                        let px_val = px.abs();

                        // println!("{}", (px_val - 1.0).abs() > f32::EPSILON);
                        if (px_val - 1.0).abs() > f32::EPSILON {
                            let result = (*px / be_width * 100.0 * 1000.0).round() / 1000.0;
                            *length = LengthValue::Vw(result);
                        }
                    }
                }
                _ => {}
            }
        }

        // if pa_option.checktype.unwrap() == "0"{
        //     if !self.in_media_condition{
        //         match length {
        //             LengthValue::Px(px) => {
        //                 let px = *px;
        //                 if px.abs() != 1.0 {
        //                     *length = LengthValue::Rem(px / font_value);
        //                 }
        //             }
        //             _ => {}
        //         }
        //     }
        // } else if pa_option.checktype.unwrap() == "1"{
        //     if !self.in_media_condition {
        //         match length {
        //             LengthValue::Px(px) => {
        //                 let px = *px;
        //                 if px.abs() != 1.0 {
        //                     let result = (px / be_width * 100.0 * 1000.0).round() / 1000.0;
        //                     *length = LengthValue::Vw(result);
        //                 }
        //             }
        //             _ => {}
        //         }
        //     }
        // }

        Ok(())
    }
}

pub fn read_file(pa_option: PaOptions) -> Result<(), Box<dyn Error>> {
    let current_dir = std::env::current_dir()?;
    let file_path = current_dir.join("css_files");
    for file_item in fs::read_dir(&file_path)? {
        let file_item = file_item?.path();
        let file_name = file_item.file_stem().unwrap().to_str().unwrap();
        // println!("file_item {:?}",file_item);
        if file_name != ".css" && !file_name.contains("_conv") {
            let mut file = File::open(&file_item)?;
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            let content = unit_analysis_change(pa_option, &content);
            let css_unit = match pa_option.checktype.unwrap_or("_") {
                "0" => "rem",
                "1" => "vw",
                _ => "_",
            };

            let new_name = format!("{}{}{}{}{}", file_name, "_", "conv", "_", css_unit);
            let new_file_path = format!(
                "{}{}{}{}",
                file_path.to_str().unwrap(),
                "\\",
                new_name,
                ".css"
            );
            let mut new_file = File::create(new_file_path)?;
            new_file.write_all(content.as_bytes())?;
        }
    }

    Ok(())
}

fn unit_analysis_change(pa_option: PaOptions, css: &str) -> String {
    // 创建一个用于存储替换后的 CSS 代码的字符串
    let mut replaced_css = String::new();

    let mut stylesheet = StyleSheet::parse(css, ParserOptions::default()).unwrap();
    let mut visitor = MyVisitor {
        in_media_condition: false,
        pa_option,
    };

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
    // println!("{:?}", res.code);

    replaced_css.push_str(&res.code);
    replaced_css
}
