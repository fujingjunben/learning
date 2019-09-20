## Rust lanaguage programming
赋值会导致ownership move
将变量传递给方法也会如此，因为相当于赋值，都是拷贝stack中的指针

如果一个可修改变量，同时通过&mut 和 &赋值的变量，只要调用的位置在
