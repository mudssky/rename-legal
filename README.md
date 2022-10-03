# rename-legal

百度网盘对一些文件名和路径名会报错,其中包含一些非法字符,而且含有emoji的也不行
除此之外,也有一些windows文件名非法的情况需要重命名.
本来这样的需求是可以用powershell实现的,但是powershell对多字节的unicode编码处理存在问题,
它默认的编码utf16占两个字节,因此类似这种emoji🐻占4个字节的会被当作两个字的长度处理,总之不能正常进行替换,也许调用c#的方法能解决,但是平时我也不用c#,我也懒得研究了
因此用rust写了这个程序,用于清除这些字符.

`rename-legal -h` 查看帮助文档
```shell
对文件名字符串进行处理的程序,用于文件重命名的时候去除非法字符串和emojis

Usage: rename-legal.exe <COMMAND>

Commands:
  check    检查字符串是否符合匹配条件,比如是否含有emoji或者windows下的非法字符
  replace  替换字符串中的非法字符
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help information
  -V, --version  Print version information
```
搭配powershell命令即可实现重命名

查看存在emoji的文件名
```powershell
ls |%{ $hasEmoji=(rename-legal.exe check $_.Name); if($hasEmoji -contains 'true') {'{0}' -f  $_.Name }}
```
然后进行重命名,为了避免危险,也可以加上`-whatIf`查看具体会重命名哪些

注意要先把powershell控制台输入输出编码调为utf8,profile里加入下面这行,因为rust输出的字符串编码是utf8格式的,总之emoji不能正常转为utf16
```powershell
# powershell控制台编码设为utf8
$OutputEncoding = [console]::InputEncoding = [console]::OutputEncoding = New-Object System.Text.UTF8Encoding
```
执行重命名
```powershell
ls |%{ $hasEmoji=(rename-legal.exe check $_.Name); if($hasEmoji -contains 'true') {   Rename-Item -LiteralPath $_.Name  (rename-legal.exe replace $_.Name) }}
```