use std::{
    error::Error,
    fmt::Display,
    fs::{self, File},
    io::{Read, Write},
    path::Path,
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
    pub file_path: Option<&'a str>,
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

        Ok(())
    }
}

// pub fn read_file(pa_option: PaOptions) -> Result<(), Box<dyn Error>> {
//     // let current_dir = std::env::current_dir()?;
//     // let file_path = current_dir.join("css_files");
//     let file_path = pa_option.file_path.unwrap_or("");
//      println!("file_path_{}",file_path);
//     for file_item in fs::read_dir(&file_path)? {
//         println!("file_item_{:?}",file_item);
//         let file_item = file_item?.path();
//         let file_name = file_item.file_stem().unwrap().to_str().unwrap();
//         println!("file_name {:?}",file_name);
//         if file_name != ".css" && !file_name.contains("_conv") {
//             let mut file = File::open(&file_item)?;
//             let mut content = String::new();
//             file.read_to_string(&mut content)?;
//             let content = unit_analysis_change(pa_option, &content);
//             let css_unit = match pa_option.checktype.unwrap_or("_") {
//                 "0" => "rem",
//                 "1" => "vw",
//                 _ => "_",
//             };

//             // let new_name = format!("{}{}{}{}{}", file_name, "_", "conv", "_", css_unit);
//             // let new_file_path = format!("{}{}{}{}", file_path, "\\", new_name, ".css");
//             let new_name = format!("{}_conv_{}", file_name, css_unit);
//             let new_file_path = Path::new(file_path).join(&new_name).with_extension("css");
//             let mut new_file = File::create(new_file_path)?;
//             new_file.write_all(content.as_bytes())?;
//         }
//     }

//     Ok(())
// }
#[derive(Debug)]
pub struct NoCssFilesFound;
impl Display for NoCssFilesFound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "No CSS files found in the directory")
    }
}
impl Error for NoCssFilesFound {}

pub fn read_file(pa_option: PaOptions) -> Result<(), Box<dyn Error>> {
    let file_path = pa_option.file_path.unwrap_or("");
    let mut css_files_processed = false;
    for file_item in fs::read_dir(&file_path)? {
        // println!("file_item_{:?}", file_item);
        let file_item = file_item?.path();
        if file_item.is_dir() {
            // 递归处理子目录
            let mut sub_options = pa_option.clone();
            let file_path = file_item.to_str().unwrap().to_string();
            // println!("file_path_{}", file_path);
            sub_options.file_path = Some(&file_path);

            // 递归调用read_file并合并结果
            match read_file(sub_options) {
                Ok(_) => css_files_processed = true,
                Err(e) => {
                    // 如果子目录中没有CSS文件，继续处理其他项
                    if e.is::<NoCssFilesFound>() {
                        continue;
                    }
                    // else {
                    //     return Err(e);
                    // }
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
                        let content = unit_analysis_change(pa_option, &content);
                        let css_unit = match pa_option.checktype.unwrap_or("_") {
                            "0" => "rem",
                            "1" => "vw",
                            _ => "_",
                        };
                        let new_name = format!("{}_conv_{}", file_name, css_unit);
                        let new_file_path =
                            Path::new(file_path).join(&new_name).with_extension("css");

                        // println!("new_file_path_{:?}", new_file_path);
                        let mut new_file = File::create(new_file_path)?;
                        new_file.write_all(content.as_bytes())?;
                        css_files_processed = true;
                    }
                }
            }
        }
    }
    if !css_files_processed {
        return Err(Box::new(NoCssFilesFound));
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
