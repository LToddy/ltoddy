## 我在mac上常用的一些脚本(在scripts目录下)

- `install-kubernetes-images-for-mac.sh`

mac上使用docker，是下载docker desktop，同时他还自带了kubernetes集群的功能。
但是他的运行需要一些docker image的支持，但是我大天朝的网速，一般你不翻墙是肯定
下载不下来的，所以这个脚本用来一键下载需要的镜像。

<small>使用方式</small>
```shell
bash install-kubernetes-images-for-mac.sh
```
