## 介绍

这是一个采用nvim-oxi开发的nvim插件，用于将unicode转换为中文汉字等字符。

## 编译与配置

```
cargo build --release
```

```
mv target/release/libnvim_unicode_converter.dylib ~/.config/nvim/lua/nvim_unicode_converter.so
cp config/unicode_converter.lua ~/.config/nvim/lua/unicode_converter.lua
```

在你的`init.lua`中添加如下配置：
```
vim.opt.runtimepath:append(',~/.config/nvim/lua')

require('unicode_converter')
```

## 使用

通过快捷键`<leader-u>`进行转换unicode为中文显示。选中下面的文本，然后按下快捷键`<leader-u>`。
```
1801\u4e07\u91cc\u957f\u57ce\u6c38\u4e0d\u5012_123445_5_3811-3826s_000004.jpg
```
就会变成：
```
1801万里长城永不倒_123445_5_3811-3826s_000004.jpg
```