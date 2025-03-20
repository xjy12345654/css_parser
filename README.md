1. 需要转换的css文件放入css_files文件夹中 。
2. 运行程序与css_files目录在同级目录下。
3. 需要转换的css文件的文件名避免含有_conv字符串，否则将被忽略无法转换。

4. vw模式情况下 正常写法将全部转换成vw,   如需要vh:   写vh(数值)来进行vh的转换
例： .foo {
  width: 32px;
  height: vh(60);
}
 