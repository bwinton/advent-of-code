//-----------------------------------------------------
// Setup.

use aoc::Day;

use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

static INPUT: &'static str = "ba dec 37 if znx != 0
zrn inc -344 if znx > -9
ffz inc -928 if kjt > 3
al inc -562 if py == 0
odo dec 294 if odo >= 0
o inc -232 if bef <= 2
al inc 536 if bef >= -7
o dec 688 if iz <= 2
sms dec 407 if kjt == 0
vg inc 245 if ije < 8
ba inc 483 if app >= -7
tsh dec -557 if vg > 242
p inc 901 if o >= -929
ga dec -352 if ffz < 8
ffz inc -525 if vg < 248
tsh inc 853 if ije == 0
iw dec -390 if qps >= -7
zrn inc 933 if ga == 352
j dec 185 if qps != -7
qps dec -340 if ije >= -3
p dec -456 if app < 3
kjt dec -831 if vg >= 246
iz inc 899 if app > -5
qps inc -579 if ije >= -2
j inc -700 if app > -1
cfl dec 943 if ga == 352
py inc 20 if os != 10
app dec 825 if zrn > 579
vg inc -283 if py <= 23
w dec 298 if al >= -29
odo dec -383 if kjt <= 7
tsh inc 784 if ije > -5
p dec -838 if tsh == 2194
j inc -992 if app != -816
ba inc 385 if iw == 390
py inc -324 if qps > -246
bef dec 407 if znx != 1
iz inc -288 if py != -294
bef inc -800 if o <= -927
p dec 471 if ga != 345
ax inc -398 if iz != 613
o inc 944 if iz == 611
sms dec -228 if iz < 613
os dec -81 if kjt <= -9
zrn dec -155 if sms != -182
sms dec 998 if bef < -399
ax inc 387 if ffz <= -521
ga inc -346 if w != -298
cfl dec 270 if j >= -1878
iw dec 306 if j >= -1880
ga inc -22 if al >= -35
sms inc -790 if sms != -1185
cfl dec 865 if kjt > -3
al dec 878 if o != 24
znx inc 924 if ije > -6
p inc 707 if py == -309
iw dec 69 if sms == -1967
w inc -808 if odo < 95
ffz inc -729 if odo <= 97
bef dec -80 if cfl >= -2079
bef inc -389 if ax > -17
ax inc -571 if zrn == 744
al inc 356 if ax <= -585
app dec -423 if tsh != 2197
p dec 192 if odo > 88
kjt dec 994 if odo <= 88
ije dec -943 if iw == 15
ga inc 152 if ba >= 859
ije dec -551 if tsh == 2194
app inc 586 if znx < 933
zrn inc 537 if os > -1
znx inc -548 if j != -1880
znx dec -696 if al == -26
znx inc 779 if app <= 191
j dec -190 if cfl >= -2085
ba inc -20 if odo > 81
iz inc 200 if ffz > -1254
ffz dec -468 if iz == 611
sms dec 7 if app > 179
sms inc 20 if bef <= -716
w dec 194 if znx < 1844
odo dec 291 if ax <= -589
p inc 22 if ax >= -586
ga inc 245 if ba > 844
app inc -610 if py <= -295
zrn inc -623 if iw > 6
bef dec 268 if odo < 98
w dec -175 if p == 1558
iz dec -359 if al != -23
al dec 356 if sms != -1953
p inc 51 if zrn < 663
qps dec -145 if py > -295
p inc -262 if bef >= -974
ax dec -561 if vg > -43
ije dec 708 if zrn > 649
sms dec 42 if ba > 847
qps dec -918 if ax > -26
kjt inc -3 if kjt == 0
zrn dec 497 if kjt >= 7
ax dec 131 if p > 1597
zrn inc 71 if zrn < 662
w dec 722 if zrn == 729
qps inc -253 if zrn == 729
os dec 946 if cfl < -2076
ba inc -558 if j >= -1687
al inc -39 if al >= -382
p inc -172 if iw >= 10
ffz dec -652 if cfl <= -2088
j dec -429 if tsh == 2194
ffz dec 697 if bef <= -979
p dec 329 if ffz > -1487
ije dec -368 if znx > 1848
qps dec -363 if ffz >= -1486
ga inc 203 if vg <= -34
p dec -118 if p != 1104
iw inc 744 if bef >= -993
sms dec 993 if cfl == -2078
tsh inc -103 if bef < -975
qps dec -644 if odo <= 90
vg inc 722 if znx != 1846
iz inc 526 if ga < 922
ax inc 274 if vg == 684
ba inc 140 if ba > 299
ax inc 982 if ije < 1156
tsh inc -684 if ga > 928
app dec 304 if bef == -984
odo inc 53 if odo <= 97
app inc -836 if bef > -992
p inc -306 if tsh < 1408
o dec -840 if znx <= 1857
tsh dec 121 if p != 806
app dec 422 if kjt != -9
bef dec -706 if ffz != -1482
ga inc 144 if os > -949
ba dec 624 if kjt != -1
o inc 542 if ije <= 1163
os dec -521 if al <= -430
zrn dec 701 if os >= -947
os dec 165 if tsh != 1283
kjt dec -219 if o <= 1407
vg inc 590 if qps < 1425
qps inc -163 if qps > 1427
al dec -316 if ga == 1074
vg dec -467 if p > 792
vg dec -356 if ba != -333
kjt inc -147 if j >= -1265
cfl inc -253 if znx > 1850
sms dec 744 if zrn != 28
p dec 180 if kjt <= 72
os dec 237 if odo <= 144
vg inc -105 if iw <= 764
al dec -436 if ffz != -1486
al dec 679 if odo >= 136
w inc 816 if ffz != -1483
kjt dec -19 if bef != -284
ije dec 525 if odo != 142
ba dec -206 if znx > 1851
ffz inc -677 if ffz != -1491
w dec 232 if bef != -278
al inc -337 if ije <= 1152
os inc 591 if bef <= -273
iw dec 956 if iw >= 756
ax dec -380 if al <= -347
w dec 27 if py >= -311
sms inc 340 if p < 628
iw inc -829 if os != -762
os dec -309 if os > -765
iz inc -780 if o >= 1397
bef dec 228 if ije < 1164
ba dec -654 if al == -344
iw inc 389 if iz != 192
zrn dec -833 if tsh != 1282
os inc -439 if iz > 185
bef inc 803 if tsh > 1294
iw inc 311 if bef > -509
o dec 975 if bef > -516
ije inc 311 if ga == 1073
bef dec -836 if znx <= 1859
w inc 881 if w >= -1855
ffz dec 97 if bef > 328
sms inc -718 if ga < 1075
ffz dec 903 if app > -1990
zrn dec 876 if p > 614
py dec 69 if ffz < -3159
zrn inc 495 if ffz >= -3168
ffz dec 297 if ba < -324
j dec -81 if iz == 190
iz inc 537 if iw >= -326
sms dec 17 if kjt <= 95
os inc 652 if qps != 1266
iw dec -813 if ax <= 1491
vg dec 861 if kjt >= 87
ffz dec 263 if os == -235
qps inc 665 if bef == 338
ije inc -294 if vg >= 544
ba inc 896 if app != -1978
ffz inc -876 if p != 618
al inc -443 if odo < 152
tsh dec -566 if al > -783
app inc -611 if zrn == 480
p dec -194 if odo > 138
py dec -877 if cfl < -2326
j dec 964 if app < -2590
o dec 975 if ga >= 1066
zrn dec 49 if qps == 1270
p inc 333 if odo == 142
iw inc 621 if o <= -543
app dec -397 if ije <= 1161
kjt inc 487 if ba <= 569
kjt dec -757 if ffz >= -3723
al dec 514 if kjt != 1336
bef inc 802 if iz == 727
bef inc -693 if j < -2133
cfl dec 0 if bef == 439
j dec -415 if o < -547
j inc 624 if ffz < -3715
sms dec -156 if iw > 1103
app inc -732 if ba == 562
kjt dec -986 if ije > 1150
o inc 723 if sms <= -3228
app dec 40 if os >= -240
os inc -118 if p <= 1154
odo dec -254 if ga <= 1078
iw dec -971 if ije > 1150
iz dec 760 if bef <= 431
zrn inc 78 if vg > 534
o inc 642 if j < -1509
p dec -3 if vg < 542
j inc -718 if w != -974
kjt dec -748 if qps <= 1274
kjt inc -349 if ga >= 1070
znx dec 441 if kjt >= 2710
bef inc 200 if tsh < 1288
o dec -838 if ije != 1156
bef dec -588 if w <= -967
vg dec 442 if ba >= 559
ax dec -437 if zrn > 501
odo inc -907 if py >= 499
p inc 596 if vg <= 100
zrn inc -136 if ije == 1154
w dec 90 if tsh <= 1284
ije dec -71 if tsh > 1284
app dec 323 if ga <= 1067
odo inc 526 if app <= -2972
znx inc 807 if vg > 89
vg inc 33 if w > -984
al inc -171 if bef >= 1221
w inc -907 if ffz < -3713
zrn inc 486 if py <= 504
cfl dec -974 if o > 1651
kjt dec 269 if tsh <= 1288
w dec 816 if ga > 1064
ba dec 159 if iw <= 2075
znx dec 319 if ije != 1222
iw inc 561 if os == -353
ije inc -269 if w != -2707
sms dec -639 if ax > 1916
odo dec -257 if p > 1741
tsh dec -489 if cfl == -1357
vg inc -188 if zrn != 862
kjt inc -203 if sms < -2583
iz dec 15 if znx >= 1895
tsh dec -987 if ax <= 1924
os dec -945 if p == 1744
ije dec 941 if bef != 1221
os dec 725 if j <= -1519
py dec -15 if iz >= 717
py inc -497 if ffz < -3715
qps dec -614 if o <= 1657
o dec -690 if p != 1736
zrn inc 953 if iw != 2645
app dec -732 if iw > 2636
al inc -374 if odo >= 277
zrn dec -286 if w <= -2695
zrn dec -642 if j >= -1521
iw dec 979 if os != 591
odo inc -124 if os == 587
kjt dec 76 if ba < 554
kjt dec 313 if znx == 1894
sms dec 614 if iz == 703
bef inc -285 if ba != 555
j inc 961 if ffz == -3720
w inc 945 if tsh == 2762
tsh dec -181 if cfl > -1367
app inc -781 if zrn != 2732
o inc 401 if tsh != 2949
znx inc 240 if ba < 571
ba dec 165 if o > 2749
ije inc 45 if p == 1744
j inc 549 if cfl != -1353
vg inc -329 if app < -3013
iw inc 847 if vg <= -377
p inc 478 if tsh < 2948
bef dec 97 if os < 587
kjt dec -256 if os > 588
znx dec 336 if os > 589
o dec -508 if kjt >= 2509
ga inc 939 if kjt < 2494
bef dec 234 if j > -14
ga dec -260 if bef <= 711
py inc -306 if zrn < 2747
kjt inc 704 if kjt >= 2505
ga dec -490 if zrn < 2743
app inc -167 if ije != 60
ffz inc -228 if app <= -3022
kjt inc -985 if znx <= 1803
bef inc -88 if ije < 64
iz inc -495 if qps != 1278
znx dec 585 if odo != 269
odo inc -211 if sms < -2584
sms inc 48 if znx == 1226
ga dec 434 if ax <= 1912
odo inc 471 if bef >= 619
p inc -758 if iz > 219
zrn dec 114 if odo != 538
zrn inc 32 if w <= -1747
kjt inc -481 if ffz < -3945
p inc 244 if ga <= 1832
ffz dec -189 if zrn >= 2656
ije dec 165 if kjt >= 1026
w inc 723 if odo >= 531
ije inc -626 if w > -1035
cfl inc -791 if cfl >= -1361
ba inc 81 if cfl <= -2140
ga dec -237 if odo < 528
vg inc 139 if p == 2466
iw inc -139 if py <= -309
o dec -100 if sms == -2589
os inc -813 if ffz < -3757
o dec 667 if iz == 212
iw dec 362 if odo > 527
odo dec -669 if os != -221
odo dec -862 if ba < 484
odo inc -240 if cfl != -2155
znx inc 109 if ga <= 1831
al inc 826 if cfl <= -2145
py inc -607 if znx >= 1324
tsh dec -280 if py < -899
iz dec -149 if w <= -1022
o inc -203 if iz < 373
odo dec 535 if kjt > 1028
odo dec -827 if py != -901
p dec -972 if cfl < -2143
os dec 578 if py <= -904
al dec -754 if odo <= 1448
znx dec 318 if p >= 3431
zrn inc 101 if py < -897
py inc -770 if tsh >= 3217
odo inc -356 if iw >= 2153
ffz inc 864 if ije == -731
os dec 401 if tsh >= 3218
cfl inc 287 if ga >= 1818
vg inc 212 if os != -1193
j inc -434 if odo != 1456
ga inc -337 if kjt <= 1025
ax dec 272 if al <= 112
bef dec 795 if kjt == 1035
al dec 21 if qps == 1270
ga dec 566 if odo > 1441
sms inc 928 if vg <= -27
al dec 646 if znx <= 1014
odo dec 905 if o <= 2649
py dec -980 if bef <= -166
ga dec -912 if o > 2645
vg inc 205 if sms >= -1665
py inc -324 if ffz < -2885
sms inc -164 if odo != 541
j dec -158 if zrn != 2763
py inc -808 if zrn > 2755
o inc -902 if o <= 2651
al inc 532 if ba == 472
sms dec 276 if tsh >= 3222
j dec -98 if odo < 538
vg inc -631 if al <= -572
znx dec -433 if tsh > 3213
bef dec -260 if al <= -556
al inc -682 if iz < 361
sms inc 121 if ax == 1649
sms dec -57 if odo != 547
os inc 280 if sms > -1763
ba inc 327 if iw != 2142
iz inc -433 if ax <= 1640
al dec 419 if app < -3022
iw inc 458 if o < 1753
tsh dec 335 if py <= -1820
w inc 384 if ax >= 1656
j inc -164 if iz != 372
bef inc -364 if odo <= 546
znx dec 72 if vg < 178
os inc 906 if al != -989
app inc -804 if p == 3443
tsh dec 293 if ga > 2178
iw dec -49 if tsh <= 2894
tsh dec 163 if ga != 2176
w dec -885 if o != 1743
ije dec -190 if qps > 1263
al dec 240 if os == -14
tsh dec 455 if odo < 547
tsh inc -137 if p <= 3441
al inc 816 if odo == 532
tsh inc -478 if j > -456
zrn inc -606 if iz != 367
j dec 770 if ije <= -537
os inc 403 if zrn == 2153
zrn dec 211 if o <= 1745
znx inc 684 if odo > 531
o dec -278 if vg <= 163
o dec 891 if cfl == -1869
iw inc 830 if ije >= -545
j inc -133 if ba <= 810
vg dec 280 if ffz <= -2895
sms inc -191 if w <= -139
tsh dec -743 if p > 3431
o inc -43 if kjt > 1033
sms dec 938 if cfl > -1867
qps dec 182 if j >= -1355
p inc -811 if iw >= 3478
kjt dec -526 if al == -1218
ax dec 399 if bef == -279
ax dec -67 if ba > 797
bef inc -916 if ije >= -536
ba dec 993 if p != 2622
py dec 445 if kjt != 1028
ga dec 862 if kjt == 1035
vg dec -951 if zrn != 1932
ba inc -757 if app > -3032
p inc -518 if ax > 1311
iw dec -441 if sms <= -2883
ije dec 603 if app < -3015
kjt dec -368 if ga == 1304
ba dec -875 if ffz <= -2897
odo inc -589 if vg <= 842
qps inc 138 if tsh == 2398
tsh inc 271 if zrn < 1939
ax dec 81 if py >= -2276
cfl dec 317 if ax != 1242
bef dec 913 if iw > 3919
iw inc -282 if o == 1702
bef dec 743 if znx == 2053
ije dec -432 if bef == -1935
py inc -512 if tsh < 2408
cfl inc 850 if app != -3025
ba dec 585 if ffz > -2899
ba dec -72 if ije <= -712
ije dec 299 if bef <= -1931
ax inc -390 if iw >= 3638
bef inc 630 if ffz > -2889
ije inc -324 if os != 379
iw dec 144 if o > 1701
py dec -99 if vg == 852
ije dec -83 if os < 390
iw dec 521 if os >= 381
ba dec 862 if iz >= 368
zrn inc -532 if zrn > 1933
os dec 957 if os >= 387
al dec -422 if odo <= -39
p dec 89 if tsh < 2400
ije dec 365 if znx > 2052
j inc 147 if odo > -56
qps dec -796 if app != -3015
os dec -512 if kjt >= 1030
sms inc 930 if os >= -59
ije dec -752 if ije >= -1625
sms dec -213 if ga == 1308
ffz inc -497 if ax < 850
cfl dec 838 if bef >= -1932
p inc -261 if al >= -800
py dec -592 if p >= 1753
w inc 77 if zrn <= 1413
ije inc 70 if bef > -1930
p dec 533 if vg == 841
vg dec -31 if sms >= -1745
zrn dec -617 if zrn == 1410
o dec 80 if iz >= 364
zrn dec 145 if zrn <= 2028
ga dec -736 if ga == 1308
ax inc 117 if tsh <= 2405
iz dec -347 if w >= -62
ba dec 7 if os < -54
py dec 756 if ba == -1465
ba inc 534 if al >= -805
j dec 19 if iw >= 2978
iz dec -993 if ba == -923
w inc -721 if cfl <= -1326
sms dec -913 if ije <= -856
bef inc -486 if w > -780
ba inc 779 if iz < 376
w dec 720 if bef < -1929
iz dec -853 if p >= 1761
cfl inc -529 if j <= -1203
al inc 81 if ije >= -855
w dec -750 if qps <= 2019
py dec 378 if ffz == -3382
sms inc -77 if p < 1762
app inc 225 if sms <= -912
kjt dec 114 if kjt >= 1032
ga dec -190 if kjt == 921
vg dec 844 if py == -2949
iz dec 280 if ga >= 2228
iw dec -789 if qps <= 2030
ije dec -16 if ax >= 962
ga inc 731 if bef >= -1935
py inc -47 if sms <= -900
al dec 121 if bef < -1928
os dec -656 if ax == 963
w dec 765 if o != 1621
znx inc -559 if zrn <= 1884
znx inc -829 if ba > -152
tsh inc -207 if al <= -912
qps dec -109 if os < 603
p dec -915 if sms <= -910
qps inc -381 if iw < 3757
znx dec -805 if p < 1764
ba inc 613 if tsh <= 2200
znx dec 202 if o >= 1620
sms dec -48 if app < -3015
qps inc -977 if ije <= -845
py dec -966 if qps < 1159
p dec 656 if cfl > -1859
ga inc -564 if p == 1103
ga inc -301 if p > 1093
ba dec -360 if app >= -3013
zrn inc 266 if j < -1195
ax inc 75 if tsh <= 2192
iz dec 889 if znx != 2097
j inc -217 if j >= -1210
py inc -226 if os != 600
app dec 326 if bef >= -1935
ga inc -810 if kjt == 921
kjt dec -500 if vg >= 25
cfl inc -954 if ije < -848
al dec -986 if w > -2283
odo inc 260 if tsh < 2195
odo inc 37 if os == 600
py inc 787 if ffz == -3394
p inc -586 if ffz >= -3391
o inc 721 if o < 1630
iz inc -556 if al != 66
vg dec 880 if al != 69
p inc -541 if qps == 1154
iw inc -735 if zrn > 2147
znx inc -373 if app == -3350
cfl dec 541 if ba <= 466
w inc -849 if ije == -849
os dec -848 if sms < -865
p dec 354 if j < -1416
qps dec 706 if al <= 68
ije inc -962 if al > 57
ga dec -918 if vg > -858
qps dec 584 if w > -3124
kjt dec 215 if cfl > -3362
sms inc -518 if tsh <= 2199
j dec 655 if ba > 452
ba inc -531 if app != -3340
cfl inc -117 if znx <= 2101
ba inc -939 if zrn >= 2149
iz dec -954 if ffz <= -3391
iw dec 863 if sms != -1384
al dec -333 if j > -2078
sms inc 906 if odo >= 244
tsh dec -971 if j <= -2068
py inc -848 if iz == 484
py inc -803 if iz != 477
ije dec 204 if ba != -71
vg inc -360 if ffz > -3394
ga dec -14 if iw != 2168
w dec -28 if j != -2077
zrn dec 947 if tsh == 3162
tsh dec -21 if ba != -78
o dec -376 if bef > -1936
os dec 78 if odo > 253
bef dec -582 if os == 610
py dec 609 if bef == -1935
ax inc 803 if ga > 2199
w inc -397 if bef == -1935
iz inc 898 if al > 401
iw inc -309 if ga <= 2198
ba dec -142 if app != -3345
p dec 489 if w == -3490
znx dec -777 if py != -4290
odo inc 416 if ga < 2212
tsh inc -779 if ffz > -3396
bef dec -696 if o != 2714
o dec -526 if odo > 667
bef inc -491 if bef <= -1232
j inc 258 if qps >= -144
qps dec -286 if iw >= 2165
ax inc 230 if app <= -3352
ax dec 714 if w != -3501
o dec -239 if ax <= 1130
qps dec -55 if bef != -1730
znx inc 847 if ba == 72
vg dec 465 if cfl >= -3478
kjt inc 821 if os >= 599
cfl inc 365 if w != -3484
sms inc 918 if py != -4295
ax dec -123 if sms <= 447
odo inc -544 if qps != 147
iw inc 733 if iw < 2171
ffz inc 679 if p != 216
ba dec 383 if app != -3347
zrn dec -22 if ffz != -2720
tsh dec -425 if vg > -1678
znx inc -427 if ffz < -2703
j inc 971 if iw >= 2900
o dec -26 if o != 2958
ffz dec -282 if ba <= -302
ga dec -709 if cfl >= -3095
p inc 57 if kjt <= 2028
odo dec 828 if w <= -3496
j dec -812 if al > 393
kjt inc -851 if znx != 2524
ba dec 575 if odo >= 118
ba dec 191 if ga < 2209
ffz inc 384 if w >= -3496
ga inc 764 if zrn > 1221
ga dec -827 if ba > -1082
ffz inc 240 if sms == 445
vg inc -398 if qps > 153
vg dec 713 if w <= -3484
ga dec -695 if iz < 492
zrn dec 358 if sms <= 449
os inc -983 if qps >= 141
py inc -834 if iz <= 492
znx inc -822 if ax <= 1251
kjt dec -138 if kjt != 1169
os dec -304 if iz < 487
py inc -924 if j == -24
j inc -100 if o >= 2962
ffz dec -645 if bef <= -1727
os dec 416 if ffz < -1152
ije dec 309 if w <= -3487
ga dec -147 if iz != 480
py dec 639 if zrn == 865
py dec 15 if sms > 440
p inc 746 if qps != 156
al dec 202 if znx >= 1689
bef dec -768 if app <= -3343
zrn inc 425 if o != 2958
qps inc 988 if zrn == 865
j dec -50 if iz > 491
iw inc 711 if ije <= -2331
znx inc -936 if zrn == 865
iz dec -253 if cfl <= -3097
sms inc 319 if al <= 205
os inc -922 if w != -3489
qps dec -156 if cfl >= -3110
ba dec -159 if kjt <= 1322
iz dec -977 if qps != 1294
iw inc 914 if p <= 1020
odo dec 195 if ffz != -1163
w inc -169 if znx != 769
j dec -782 if kjt != 1310
zrn inc -746 if ije <= -2317
ga dec 819 if ffz > -1171
qps dec -897 if kjt >= 1309
ga inc -267 if o <= 2966
znx inc 35 if py >= -5786
qps inc -963 if py < -5774
app inc 765 if vg <= -2382
w inc 738 if znx > 802
app dec 80 if al >= 188
sms dec 799 if tsh != 2819
tsh inc -853 if qps >= 1223
ax dec 442 if vg > -2399
qps dec -341 if zrn >= 115
odo dec -166 if znx <= 801
iz dec -254 if w <= -3662
app inc 662 if sms == -35
zrn inc 982 if ax <= 810
o inc -671 if kjt > 1309
iz dec 496 if iw != 3805
iw dec -840 if iw > 3808
ba dec -692 if qps >= 1565
j dec -601 if qps != 1561
cfl dec -861 if znx >= 793
app dec -928 if ije == -2324
iz inc -535 if j > 1353
vg inc -367 if app < -1082
al inc -788 if sms != -30
ffz dec 398 if znx <= 786
w inc -881 if tsh == 1976
vg dec 711 if al > -594
w dec -509 if odo == 88
cfl dec 360 if cfl > -2253
kjt inc 505 if znx >= 793
bef inc -468 if py == -5778
j dec -653 if zrn <= 1104
vg inc -674 if os < -1424
odo dec 283 if ba == -226
ga inc 227 if ffz == -1162
ije dec 276 if vg >= -3097
j inc -159 if ffz >= -1164
p dec 857 if ax >= 802
tsh inc 519 if os <= -1414
ga dec -54 if ba != -219
ije dec 821 if w != -4543
qps dec -704 if j >= 1841
cfl inc -392 if ga >= 3843
qps dec 156 if iw < 4656
iw dec 505 if cfl != -2604
o dec -98 if al < -588
sms inc 277 if tsh != 2500
py dec -874 if ba <= -220
sms inc 102 if kjt >= 1818
bef inc -577 if ga >= 3833
app inc -285 if kjt != 1819
app dec -776 if odo == -191
ije inc 796 if py >= -4913
znx inc -883 if iz >= 237
vg inc 692 if vg <= -3098
sms inc -100 if ga == 3836
p dec 520 if tsh > 2497
vg dec 275 if vg >= -2402
zrn dec 418 if odo < -187
w dec -313 if znx == -89
o dec 705 if p < 155
w inc 11 if qps >= 2112
ffz inc -104 if os >= -1424
sms inc 365 if ax > 798
qps inc -865 if kjt > 1824
ax dec 897 if vg == -2408
ba dec -391 if bef <= -2004
os dec 36 if os >= -1413
o inc 580 if tsh >= 2487
iz dec 475 if sms < 613
app inc 382 if ffz >= -1262
bef dec -763 if o <= 2267
vg inc 387 if sms < 604
os inc -441 if qps != 2123
o inc -84 if iz > -239
iz dec -82 if app < -307
j inc 937 if vg < -2402
vg dec 726 if os > -1856
ga inc -739 if ffz == -1266
zrn dec -721 if odo != -199
py inc -154 if p == 154
al inc 482 if al < -587
znx inc 967 if vg >= -2409
j inc 991 if py != -5063
kjt dec -438 if p <= 159
odo dec -308 if os >= -1854
w dec 943 if zrn == 1404
iz dec -855 if py == -5058
cfl inc -45 if qps < 2126
zrn dec -993 if qps > 2126
qps dec -567 if ije == -2349
kjt inc 848 if iz <= 621
ffz inc -194 if ffz < -1261
iz inc 172 if znx >= 869
kjt dec -138 if ba <= 162
app inc -4 if zrn > 1396
ga inc -607 if tsh != 2493
sms dec -300 if w > -5169
ba dec -147 if ba != 166
ffz dec -962 if app == -302
os inc -626 if iw > 4155
znx inc 235 if znx == 878
bef inc -995 if zrn >= 1404
ba dec -197 if ga < 2500
tsh dec 592 if sms >= 905
ax inc 300 if zrn != 1404
sms dec -910 if bef > -2248
zrn inc -350 if p != 164
iw dec 225 if o != 2176
p inc 856 if bef != -2239
ije dec -673 if w == -5169
znx inc -24 if ba > 499
ga inc 858 if znx > 1080
o dec 751 if tsh != 1909
kjt dec -860 if vg == -2407
znx inc 506 if iw > 4146
os dec -434 if ba != 505
kjt dec 644 if odo <= -182
zrn inc 674 if ba >= 501
ga dec -966 if j != 3773
ije dec -182 if al == -110
ba inc 967 if vg > -2401
znx dec -527 if j >= 3765
qps dec 176 if sms >= 1818
p inc -817 if al < -100
ffz dec -232 if odo < -188
app dec -538 if w <= -5155
ffz dec -702 if odo < -200
qps dec -196 if ax >= -96
ga inc -534 if sms <= 1811
ije dec -525 if ga == 4314
bef dec 105 if py >= -5061
ga inc -485 if al < -111
app inc -461 if ax < -80
qps dec -482 if sms > 1818
al dec 968 if cfl >= -2654
ije inc 750 if p != -663
odo dec 207 if cfl < -2647
al inc 733 if w == -5160
o inc 211 if ga <= 4310
j inc 110 if py >= -5056
al inc 777 if tsh < 1909
zrn dec 851 if tsh == 1903
odo dec -241 if j <= 3772
qps inc 285 if znx <= 2117
os dec 547 if znx > 2114
o inc -650 if w > -5156
p inc 264 if py > -5061
sms inc -850 if j != 3763
znx dec -405 if iw != 4157
kjt dec -927 if qps == 3186
al dec 119 if ga < 4320
j dec -862 if qps > 3176
ba dec 166 if tsh >= 1900
ba inc -622 if qps != 3186
w inc -777 if w != -5160
j inc -860 if ba < 346
znx dec 798 if iw > 4149
cfl dec 218 if bef <= -2339
al inc -504 if tsh == 1903
qps dec -646 if ax > -90
ffz dec -687 if ffz <= -261
qps inc -602 if o < 1435
w inc 358 if sms != 969
o inc -941 if p == -399
znx dec -32 if app == -225
qps inc -356 if znx > 1758
j inc 287 if iw > 4145
kjt inc -18 if zrn >= 870
sms inc -917 if kjt > 3369
o dec 173 if w > -5170
app inc -144 if tsh <= 1909
bef inc 105 if ba > 334
tsh dec 855 if app > -370
iw dec -504 if sms < 46
iw dec 666 if os < -1969
vg dec -61 if o > 308
ffz dec -73 if qps != 2872
iz inc -498 if kjt != 3367
ije dec 53 if p >= -400
py inc -955 if znx < 1770
p inc -282 if j == 4060
cfl dec 237 if os <= -1967
app dec 203 if o > 317
o inc -201 if w == -5160
iw dec 234 if zrn <= 883
ije inc 493 if al > -193
w dec 971 if qps == 2874
app dec -415 if qps >= 2870
app inc -775 if al <= -187
j dec -878 if iz == 295
w inc 15 if ax != -99
vg inc 317 if qps == 2874
o dec 26 if ga < 4315
cfl dec -200 if ba > 336
iw dec 741 if app == -729
znx inc -134 if znx >= 1754
iw dec 464 if zrn == 877
tsh inc -908 if bef == -2239
j inc 202 if cfl > -2907
app inc 0 if znx != 1627
py dec 414 if iz >= 287
qps inc -384 if app > -732
sms dec 915 if ba == 343
iz dec 797 if tsh <= 141
iw inc -909 if bef < -2235
odo dec -300 if os > -1966
ax inc 829 if iz < -498
qps dec -169 if al <= -191
o dec -464 if tsh < 148
bef dec 312 if al > -201
w dec -299 if iw >= 1146
ffz dec -37 if w <= -6119
bef inc 169 if bef == -2551
cfl inc -364 if py >= -6427
ffz dec -199 if ije >= -1197
qps dec -287 if znx != 1634
ba inc 108 if ije > -1208
znx dec -765 if w > -6111
j inc -138 if ax <= 744
iz inc -26 if znx >= 1627
qps inc -397 if py == -6427
ffz dec -550 if al == -191
kjt inc 776 if w >= -6116
ba dec 743 if w > -6123
ax inc -644 if al < -188
cfl inc -824 if app <= -732
odo inc -830 if odo >= -147
znx dec 52 if ffz < 1049
j inc -576 if znx < 1582
odo dec -58 if ax != 96
j dec 212 if tsh != 143
kjt dec 980 if j >= 4205
odo inc -463 if zrn != 877
ga inc 607 if o >= 548
ax inc -946 if o <= 542
ga inc 715 if p <= -678
py inc -549 if odo >= -163
vg inc -148 if iw <= 1141
py inc -633 if py != -6978
w inc 71 if kjt <= 3168
iw dec -433 if zrn < 880
iw inc 280 if bef > -2391
app inc 453 if os == -1971
j inc 137 if vg == -2178
ba inc 680 if ffz >= 1037
ga inc -233 if odo <= -150
ba dec -122 if o <= 549
ga dec -169 if znx > 1570
zrn inc -252 if p >= -687
p dec -129 if znx == 1575
py dec -547 if iz > -522
iw dec -796 if ba > 506
tsh inc -179 if vg < -2170
iw inc -650 if sms == -863
ffz inc -768 if ije < -1192
j inc -324 if iz > -530
o dec -37 if ije >= -1208
bef dec 76 if ga == 5568
tsh inc -287 if app >= -285
qps inc -132 if znx < 1583
tsh dec -12 if vg >= -2181
ije dec -394 if py >= -7612
ba dec -39 if p != -557
zrn inc -728 if o > 578
ije inc -468 if j == 4027
o inc 414 if odo != -164
app dec -601 if j != 4024
iz dec -203 if znx >= 1572
iw dec -785 if app == 325
zrn inc 987 if vg < -2177
bef dec -158 if w == -6045
ffz inc -535 if sms >= -872
al inc -992 if o <= 1002
tsh inc 575 if j != 4019
qps inc -85 if vg == -2178
py dec 235 if w == -6045
sms dec -882 if j != 4025
iz inc -869 if ba != 548
ga dec -964 if ax < 89
ffz dec 758 if ga <= 5580
kjt dec -48 if tsh >= 263
ga dec 807 if py < -7838
bef dec -763 if p < -546
py dec -648 if o > 990
vg dec 456 if al <= -1188
odo dec -95 if app != 318
bef dec -7 if cfl <= -3277
vg dec 343 if zrn <= 890
cfl inc 273 if qps >= 2326
j dec -459 if ije >= -1284
py dec 210 if ije >= -1283
zrn dec 527 if ax > 88
al inc -14 if odo > -65
al dec -358 if bef < -1455
cfl inc -83 if ax >= 89
j dec -690 if sms <= 25
py dec -752 if znx > 1574
kjt dec 609 if tsh >= 261
ba dec -362 if bef != -1461
sms inc 468 if iz < -1193
ije dec 85 if qps > 2324
ax inc -763 if app == 325
o inc 681 if w == -6045
ba inc -963 if cfl == -3077
ffz inc -945 if py >= -6659
j dec 224 if o != 1674
o inc 227 if py >= -6654
znx inc 717 if ffz != -1952
ba inc 787 if ga < 4763
iw inc -248 if bef == -1461
tsh dec 29 if cfl <= -3074
ga dec 968 if py <= -6649
iz dec 420 if ffz <= -1955
sms dec 27 if o < 1910
cfl inc 23 if bef <= -1459
ga inc 20 if zrn > 348
j dec 385 if iz > -1617
tsh inc -305 if p <= -549
iz inc 53 if j >= 4561
w dec -261 if sms < 465
al dec 887 if iz < -1558
w inc -131 if os != -1973
ffz inc -220 if py >= -6651
al inc -746 if app != 325
ba inc -488 if ffz == -1962
vg inc -694 if py > -6655
qps inc -647 if iw <= 2538
w dec -514 if bef <= -1455
py dec 529 if py != -6654
tsh dec 720 if tsh != -74
ba inc -466 if os != -1972
ga inc 662 if w <= -5402
sms dec 356 if ije < -1353
al dec -62 if app < 328
bef dec -381 if app > 320
zrn inc 106 if vg <= -3216
kjt dec 925 if py > -6646
cfl inc 699 if w <= -5399
iw inc 549 if ga > 3824
iz inc 653 if zrn < 356
ba dec 674 if ije <= -1362
zrn inc -255 if ba < -1363
ax dec 329 if ga <= 3826";

#[derive(Clone)]
#[derive(Debug)]
enum Operation {
  Inc(i32),
  Dec(i32),
}

impl FromStr for Operation {
  type Err = ();

  fn from_str(s: &str) -> Result<Operation, ()> {
    lazy_static! {
      static ref MAIN_RE: Regex = Regex::new(r"^(inc|dec) (-?\d+)?$").unwrap();
    }
    let cap = MAIN_RE.captures(s);
    match cap {
      None => Err(()),
      Some(x) => {
        let value = x[2].parse().unwrap();
        match &x[1] {
          "inc" => Ok(Operation::Inc(value)),
          "dec" => Ok(Operation::Dec(value)),
          _ => {
            println!("Unknown Operation {}", s);
            Err(())
          },
        }
      },
    }
  }
}

#[derive(Clone)]
#[derive(Debug)]
enum Test {
  Greater,
  GreaterEqual,
  Equal,
  NotEqual,
  LessEqual,
  Less,
}

impl FromStr for Test {
  type Err = ();

  fn from_str(s: &str) -> Result<Test, ()> {
    match s {
      ">" => Ok(Test::Greater),
      ">=" => Ok(Test::GreaterEqual),
      "==" => Ok(Test::Equal),
      "!=" => Ok(Test::NotEqual),
      "<=" => Ok(Test::LessEqual),
      "<" => Ok(Test::Less),
      _ => {
        println!("Unknown Test {}", s);
        Err(())
      },
    }
  }
}

#[derive(Clone)]
#[derive(Debug)]
struct Condition {
  source: String,
  test: Test,
  value: i32,
}

impl FromStr for Condition {
  type Err = ();

  fn from_str(s: &str) -> Result<Condition, ()> {
    lazy_static! {
      static ref MAIN_RE: Regex = Regex::new(r"^([a-z]+) ([!<>=]=?) (-?\d+)$").unwrap();
    }
    let cap = MAIN_RE.captures(s);
    match cap {
      None => {
        println!("Unknown Condition {}", s);
        Err(())
      },
      Some(x) => {
        Ok(Condition {
          source: x[1].to_string(),
          test: x[2].parse().unwrap(),
          value: x[3].parse().unwrap(),
        })
      },
    }
  }
}

impl Condition {
  fn evaluate(&self, regs: &mut HashMap<String, i32>) -> bool {
    let reg = regs.entry(self.source.clone()).or_insert(0);
    match self.test {
      Test::Greater => *reg > self.value,
      Test::GreaterEqual => *reg >= self.value,
      Test::Equal => *reg == self.value,
      Test::NotEqual => *reg != self.value,
      Test::LessEqual => *reg <= self.value,
      Test::Less => *reg < self.value,
    }
  }
}


#[derive(Clone)]
#[derive(Debug)]
struct Instruction {
  dest: String,
  op: Operation,
  cond: Condition,
}

impl FromStr for Instruction {
  type Err = ();

  fn from_str(s: &str) -> Result<Instruction, ()> {
    lazy_static! {
      static ref MAIN_RE: Regex = Regex::new(r"^([a-z]+) ([a-z]+ -?\d+) if (([a-z]+) ..? -?\d+)$").unwrap();
    }
    let cap = MAIN_RE.captures(s);
    match cap {
      None => {
        println!("Unknown Instruction {}", s);
        Err(())
      },
      Some(x) => {
        Ok(Instruction {
          dest: x[1].to_string(),
          op: x[2].parse().unwrap(),
          cond: x[3].parse().unwrap(),
        })
      },
    }
  }
}

impl Instruction {
  fn execute(&self, regs: &mut HashMap<String, i32>) -> Option<(String, i32)> {
    if self.cond.evaluate(regs) {
      let reg = regs.entry(self.dest.clone()).or_insert(0);
      match self.op {
        Operation::Inc(value) => *reg += value,
        Operation::Dec(value) => *reg -= value,
      }
      Some((self.dest.clone(), *reg))
    } else {
      None
    }
  }
}

fn process_data_a(data: &str) -> HashMap<String, i32> {
  let mut instructions = Vec::new();
  let mut regs: HashMap<String, i32> = HashMap::new();
  for line in data.lines() {
    let instruction: Instruction = line.parse().unwrap();
    instruction.execute(&mut regs);
    instructions.push(instruction);
  }
  regs
}

fn process_data_b(data: &str) -> (String, i32) {
  let mut instructions = Vec::new();
  let mut regs: HashMap<String, i32> = HashMap::new();
  let mut max = ("a".to_string(), 0);
  for line in data.lines() {
    let instruction: Instruction = line.parse().unwrap();
    if let Some(new) = instruction.execute(&mut regs) {
      if new.1 > max.1 {
        max = new;
      }
    }
    instructions.push(instruction);
  }
  max
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl Day for Q {
  fn number(&self) -> String {
    String::from("8")
  }

  fn a(&self) {
    print!("{}A: ", self.number());
    let result = process_data_a(INPUT);
    println!("Result = {}", result.values().max().unwrap());
  }

  fn b(&self) {
    print!("{}B: ", self.number());
    let result = process_data_b(INPUT);
    println!("Result = {}", result.1);
  }
}

#[test]
fn a() {
  let expected =
    hashmap!{
    "a".to_string() => 1,
    "b".to_string() => 0,
    "c".to_string() => -10,
  };
  assert_eq!(
    process_data_a(
      "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10",
    ),
    expected
  );
}

#[test]
fn b() {
  assert_eq!(
    process_data_b(
      "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10",
    ),
    ("c".to_string(), 10)
  );
}
