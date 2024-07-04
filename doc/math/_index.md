---
title: Math 数学
math: true
---

+ 复数
+ 离散傅立叶变换DFT
+ $ \pi $
## 复数
形如`a+bi`的数为复数 a为`实部` b为`虚部` i为`虚数单位`

复数一般用`z`表示 `z = a + bi`

### 加法
`(a + bi) + (c + di) = (a + c) + (b + d)i`
### 乘法
`(a + bi) * (c + di) = (a * c - b * d) + (a * d + b * c)i`
### 除法
`a + bi / c + di = (a * c + b * d)/ (c * c + d * d) + ((b * c - a * d)/(c * c + d * d))i`

## 离散傅里叶变换
离散傅里叶变换将时间域

离散傅里叶变换 是傅里叶变换在时域和频域上都呈现离散的形式 将时域信号的采样变换为在离散时间傅里叶变换频域的采样

$ X[k] = \sum_{0}^{N-1} x[n] \cdot e^{-i \frac{2\pi}{N} nk}  $
 
 对 $ e^{-i \frac{2\pi}{N} nk} $ 进行欧拉变换为 $ cos (\frac{2\pi}{N} nk) + sin (\frac{2\pi}{N} nk) i $ 

## $ \pi $
使用莱布尼兹级数逼近pi

$ \frac{\pi}{4} =  1 - \frac{1}{3} + \frac{1}{5} - \frac{1}{7} + \frac{1}{9} - \ldots $


