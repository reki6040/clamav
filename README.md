# ClamAV

最初の説明
このリポジトリは以下のClamAV本家に改造を加えてマージしたものです。
https://github.com/Cisco-Talos/clamav

試験はctestをpassしただけであまりしてないのでほかにバグがある場合もあることも鑑みて、無保証・無担保でお願いいたします。

改造の経緯は自分用のLinuxへインストールしたときに下記のサイトの方と同じくCPU使用率の急増と通知問題に悩まされたので変更した。

改造内容は以下の2つ。
１．https://linuxfun.org/2020/12/12/clamonacc-clamd-cpu100percent/#i-2
ー＞CPU使用率急増を軽減する ファイル読み取り時にチェックする動作を変更
２．https://linuxfun.org/2021/09/29/how-to-setup-virusscan-server-by-clamav/
ー＞通知問題 ウィルス検知のメール通知時ファイルパスを通知できる対応

↑の内1点目はgithubの本家サイトでマージの議論をしていたようですが、スクリプト実行に問題があるため、本家でのマージ見送りしているようです。
私の方ではshellscript等実行しても同じようにscript実行が止まる事が発生していないので環境問題の可能性とみて、私の方では適用しました。

今後のモチベーション次第になるが、本家の安定版のリリースに従いUpdateをかけていきたい。
現在のマージ元のVersion:1.3.0 data:2023/02/23,木曜日
現在のマージVersion:1.3.0.0とした。

補足
末尾の.0が自分オリジナルの版数とする。
1.3.0までが本家のversion。
なお、本家が0.10x.x.xまでのようにしてきたら、どうするか別途考えるものとする。
詳しく見てないがcmakeでversionフォーマット変えるとエラーになり先に進ませてもらえないのでこれで今は勘弁してほしい。

マージは以下のようにしていく。
1.一度本家のものを全部コピー
2.こちらで変更した内容を再度マージする。
結果として本家のもの+自分で対応した変更とする。

Licensing
ClamAV is licensed for public/open source use under the GNU General Public License, Version 2 (GPLv2).

See COPYING.txt for a copy of the license.

3rd Party Code
ClamAV contains a number of components that include code copied in part or in whole from 3rd party projects and whose code is not owned by Cisco and which are licensed differently than ClamAV. These include:

Yara: Apache 2.0 license
Yara has since switched to the BSD 3-Clause License; Our source is out-of-date and needs to be updated.
7z / lzma: public domain
libclamav's NSIS/NulSoft parser includes:
zlib: permissive free software license
bzip2 / libbzip2: BSD-like license
OpenBSD's libc/regex: BSD license
file: BSD license
str.c: Contains BSD licensed modified-implementations of strtol(), stroul() functions, Copyright (c) 1990 The Regents of the University of California.
pngcheck (png.c): MIT/X11-style license
getopt.c: MIT license
Curl: license inspired by MIT/X, but not identical
libmspack: LGPL license
UnRAR (libclamunrar): a non-free/restricted open source license
Note: The UnRAR license is incompatible with GPLv2 because it contains a clause that prohibits reverse engineering a RAR compression algorithm from the UnRAR decompression code. For this reason, libclamunrar/libclamunrar_iface is not linked at all with libclamav. It is instead loaded at run-time. If it fails to load, ClamAV will continue running without RAR support.
See the COPYING directory for a copy of the 3rd party project licenses.

