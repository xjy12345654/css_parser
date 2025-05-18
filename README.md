**功能： 将 px 转换为 rem 或 vw/vh。**<br>
**Rust_version**: 1.85<br>
**GUI**: Slint 1.10.0<br>
**cssparser**: lightningcss(rust_lib)<br>

![图片描述](./ui/images/home.png)

**编译**需安装Rust开发环境<br>
**启动**cargo run<br>
**生成优化后的可执行程序**  cargo build --release<br>
<br>
注意事项:
1. 媒体查询 @media screen and (max-width: 300px)的范围设置不进行单位转换，1px 的绝对值不进行单位转换，当然可以根据情况修改。
2. 需要转换的 css 文件的文件名避免含有\_conv_rem 或者\_conv_vw 字符串，否则将被忽略无法转换。
3. vw 模式情况下 正常写法将全部转换成 vw, 如需要 vh: 写 vh(数值)来进行 vh 的转换
   例： .foo {
   width: 32px;
   height: vh(60);
   }
