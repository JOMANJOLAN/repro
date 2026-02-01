### 🎨 PPMDEMO - 程序化着色器动画
* **日期**：2026/01/25
* **使用语言**：C、C++、Rust
* **工具链**：FFmpeg、MPV
  ```shell
  ffmpeg -i frame-%02d.ppm video.mp4
  mpv video.mp4
  ```
* **学习历程**：
  * 使用C语言编写生成棋盘平移效果动画的简单程序
  * 使用C++移植GLSL算法，生成着色器动画
  * 使用Rust移植C++算法，并使用多线程加速
* **核心特性**：
  * 生成纹理
  * 向量运算
  * 多线程
  * PPM图像格式输出
* **基本算法**：
  1. 定义图像的大小
  2. 遍历图像每一个像素，使用特定的算法为像素着色
  3. 将图像输出为ppm格式
  4. 将每一帧生成的图像使用FFmpeg拼接为视频
* **问题**
  * **为什么使用多线程，为什么可以使用多线程？**
    * 串行算法慢
    * 帧与帧不相关
  * **Rust语言的IO为什么比C/C++语言慢这么多？**
    * C/C++的FILE结构体有缓冲区，Rust的File没有
    * 应该使用BufWriter或BufReader
  * **MPV如何设置循环播放？**
    1. 创建`~/.config/mpv/mpv.conf`文件
    2. 无限循环单个文件`loop-file=inf`
    3. 无限循环播放列表`loop-playlist=inf`
