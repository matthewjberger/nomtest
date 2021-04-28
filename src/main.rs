use nom::*;

fn main() {}

const FONT: &str = &r#"
info face="Arial Black Standard" size=32 bold=0 italic=0 charset="" unicode=0 stretchH=100 smooth=1 aa=1 padding=4,4,4,4 spacing=-8,-8
common lineHeight=46 base=36 scaleW=512 scaleH=512 pages=1 packed=0
page id=0 file="sdf.png"
chars count=97
char id=0     x=237  y=102  width=25   height=29   xoffset=-4   yoffset=11   xadvance=24   page=0    chnl=0 
char id=10    x=0    y=0    width=0    height=0    xoffset=-4   yoffset=0    xadvance=0    page=0    chnl=0 
char id=32    x=0    y=0    width=0    height=0    xoffset=-4   yoffset=0    xadvance=11   page=0    chnl=0 
char id=33    x=27   y=102  width=15   height=32   xoffset=-4   yoffset=8    xadvance=11   page=0    chnl=0 
char id=34    x=298  y=134  width=24   height=17   xoffset=-4   yoffset=8    xadvance=16   page=0    chnl=0 
char id=35    x=154  y=102  width=28   height=32   xoffset=-4   yoffset=8    xadvance=21   page=0    chnl=0 
char id=36    x=150  y=0    width=28   height=37   xoffset=-4   yoffset=6    xadvance=21   page=0    chnl=0 
char id=37    x=117  y=102  width=37   height=32   xoffset=-4   yoffset=8    xadvance=32   page=0    chnl=0 
char id=38    x=182  y=102  width=34   height=32   xoffset=-4   yoffset=8    xadvance=28   page=0    chnl=0 
char id=39    x=322  y=134  width=15   height=17   xoffset=-4   yoffset=8    xadvance=9    page=0    chnl=0 
char id=40    x=20   y=0    width=18   height=38   xoffset=-4   yoffset=8    xadvance=12   page=0    chnl=0 
char id=41    x=38   y=0    width=19   height=38   xoffset=-4   yoffset=8    xadvance=12   page=0    chnl=0 
char id=42    x=279  y=134  width=19   height=21   xoffset=-4   yoffset=8    xadvance=18   page=0    chnl=0 
char id=43    x=186  y=134  width=26   height=26   xoffset=-4   yoffset=11   xadvance=21   page=0    chnl=0 
char id=44    x=264  y=134  width=15   height=21   xoffset=-4   yoffset=25   xadvance=11   page=0    chnl=0 
char id=45    x=394  y=134  width=18   height=14   xoffset=-4   yoffset=20   xadvance=11   page=0    chnl=0 
char id=46    x=363  y=134  width=15   height=15   xoffset=-4   yoffset=25   xadvance=11   page=0    chnl=0 
char id=47    x=83   y=102  width=17   height=32   xoffset=-4   yoffset=8    xadvance=9    page=0    chnl=0 
char id=48    x=0    y=102  width=27   height=32   xoffset=-4   yoffset=8    xadvance=21   page=0    chnl=0 
char id=49    x=261  y=70   width=22   height=32   xoffset=-4   yoffset=8    xadvance=21   page=0    chnl=0 
char id=50    x=283  y=70   width=27   height=32   xoffset=-4   yoffset=8    xadvance=21   page=0    chnl=0 
char id=51    x=310  y=70   width=27   height=32   xoffset=-4   yoffset=8    xadvance=21   page=0    chnl=0 
char id=52    x=337  y=70   width=29   height=32   xoffset=-4   yoffset=8    xadvance=21   page=0    chnl=0 
char id=53    x=366  y=70   width=27   height=32   xoffset=-4   yoffset=8    xadvance=21   page=0    chnl=0 
char id=54    x=393  y=70   width=27   height=32   xoffset=-4   yoffset=8    xadvance=21   page=0    chnl=0 
char id=55    x=420  y=70   width=27   height=32   xoffset=-4   yoffset=8    xadvance=21   page=0    chnl=0 
char id=56    x=447  y=70   width=27   height=32   xoffset=-4   yoffset=8    xadvance=21   page=0    chnl=0 
char id=57    x=474  y=70   width=27   height=32   xoffset=-4   yoffset=8    xadvance=21   page=0    chnl=0 
char id=58    x=171  y=134  width=15   height=26   xoffset=-4   yoffset=14   xadvance=11   page=0    chnl=0 
char id=59    x=68   y=102  width=15   height=32   xoffset=-4   yoffset=14   xadvance=11   page=0    chnl=0 
char id=60    x=262  y=102  width=26   height=27   xoffset=-4   yoffset=11   xadvance=21   page=0    chnl=0 
char id=61    x=238  y=134  width=26   height=22   xoffset=-4   yoffset=13   xadvance=21   page=0    chnl=0 
char id=62    x=288  y=102  width=26   height=27   xoffset=-4   yoffset=11   xadvance=21   page=0    chnl=0 
char id=63    x=42   y=102  width=26   height=32   xoffset=-4   yoffset=8    xadvance=20   page=0    chnl=0 
char id=64    x=178  y=0    width=33   height=35   xoffset=-4   yoffset=8    xadvance=24   page=0    chnl=0 
char id=65    x=244  y=0    width=33   height=32   xoffset=-4   yoffset=8    xadvance=25   page=0    chnl=0 
char id=66    x=277  y=0    width=30   height=32   xoffset=-4   yoffset=8    xadvance=25   page=0    chnl=0 
char id=67    x=307  y=0    width=30   height=32   xoffset=-4   yoffset=8    xadvance=25   page=0    chnl=0 
char id=68    x=337  y=0    width=30   height=32   xoffset=-4   yoffset=8    xadvance=25   page=0    chnl=0 
char id=69    x=367  y=0    width=28   height=32   xoffset=-4   yoffset=8    xadvance=23   page=0    chnl=0 
char id=70    x=395  y=0    width=26   height=32   xoffset=-4   yoffset=8    xadvance=21   page=0    chnl=0 
char id=71    x=421  y=0    width=33   height=32   xoffset=-4   yoffset=8    xadvance=27   page=0    chnl=0 
char id=72    x=454  y=0    width=31   height=32   xoffset=-4   yoffset=8    xadvance=27   page=0    chnl=0 
char id=73    x=485  y=0    width=16   height=32   xoffset=-4   yoffset=8    xadvance=12   page=0    chnl=0 
char id=74    x=0    y=38   width=26   height=32   xoffset=-4   yoffset=8    xadvance=21   page=0    chnl=0 
char id=75    x=26   y=38   width=33   height=32   xoffset=-4   yoffset=8    xadvance=27   page=0    chnl=0 
char id=76    x=59   y=38   width=26   height=32   xoffset=-4   yoffset=8    xadvance=21   page=0    chnl=0 
char id=77    x=85   y=38   width=34   height=32   xoffset=-4   yoffset=8    xadvance=30   page=0    chnl=0 
char id=78    x=119  y=38   width=31   height=32   xoffset=-4   yoffset=8    xadvance=27   page=0    chnl=0 
char id=79    x=150  y=38   width=33   height=32   xoffset=-4   yoffset=8    xadvance=27   page=0    chnl=0 
char id=80    x=183  y=38   width=27   height=32   xoffset=-4   yoffset=8    xadvance=23   page=0    chnl=0 
char id=81    x=211  y=0    width=33   height=34   xoffset=-4   yoffset=8    xadvance=27   page=0    chnl=0 
char id=82    x=210  y=38   width=31   height=32   xoffset=-4   yoffset=8    xadvance=25   page=0    chnl=0 
char id=83    x=241  y=38   width=30   height=32   xoffset=-4   yoffset=8    xadvance=23   page=0    chnl=0 
char id=84    x=271  y=38   width=30   height=32   xoffset=-4   yoffset=8    xadvance=23   page=0    chnl=0 
char id=85    x=301  y=38   width=31   height=32   xoffset=-4   yoffset=8    xadvance=27   page=0    chnl=0 
char id=86    x=332  y=38   width=33   height=32   xoffset=-4   yoffset=8    xadvance=25   page=0    chnl=0 
char id=87    x=365  y=38   width=41   height=32   xoffset=-4   yoffset=8    xadvance=32   page=0    chnl=0 
char id=88    x=406  y=38   width=33   height=32   xoffset=-4   yoffset=8    xadvance=25   page=0    chnl=0 
char id=89    x=439  y=38   width=33   height=32   xoffset=-4   yoffset=8    xadvance=25   page=0    chnl=0 
char id=90    x=472  y=38   width=29   height=32   xoffset=-4   yoffset=8    xadvance=23   page=0    chnl=0 
char id=91    x=57   y=0    width=19   height=38   xoffset=-4   yoffset=8    xadvance=12   page=0    chnl=0 
char id=92    x=100  y=102  width=17   height=32   xoffset=-4   yoffset=8    xadvance=9    page=0    chnl=0 
char id=93    x=76   y=0    width=19   height=38   xoffset=-4   yoffset=8    xadvance=12   page=0    chnl=0 
char id=94    x=212  y=134  width=26   height=22   xoffset=-4   yoffset=8    xadvance=21   page=0    chnl=0 
char id=95    x=412  y=134  width=25   height=11   xoffset=-4   yoffset=33   xadvance=16   page=0    chnl=0 
char id=96    x=378  y=134  width=16   height=14   xoffset=-4   yoffset=8    xadvance=11   page=0    chnl=0 
char id=97    x=314  y=102  width=26   height=26   xoffset=-4   yoffset=14   xadvance=21   page=0    chnl=0 
char id=98    x=0    y=70   width=26   height=32   xoffset=-4   yoffset=8    xadvance=21   page=0    chnl=0 
char id=99    x=340  y=102  width=26   height=26   xoffset=-4   yoffset=14   xadvance=21   page=0    chnl=0 
char id=100   x=26   y=70   width=26   height=32   xoffset=-4   yoffset=8    xadvance=21   page=0    chnl=0 
char id=101   x=366  y=102  width=27   height=26   xoffset=-4   yoffset=14   xadvance=21   page=0    chnl=0 
char id=102   x=52   y=70   width=21   height=32   xoffset=-4   yoffset=8    xadvance=12   page=0    chnl=0 
char id=103   x=73   y=70   width=26   height=32   xoffset=-4   yoffset=14   xadvance=21   page=0    chnl=0 
char id=104   x=99   y=70   width=25   height=32   xoffset=-4   yoffset=8    xadvance=21   page=0    chnl=0 
char id=105   x=124  y=70   width=15   height=32   xoffset=-4   yoffset=8    xadvance=11   page=0    chnl=0 
char id=106   x=0    y=0    width=20   height=38   xoffset=-4   yoffset=8    xadvance=11   page=0    chnl=0 
char id=107   x=139  y=70   width=27   height=32   xoffset=-4   yoffset=8    xadvance=21   page=0    chnl=0 
char id=108   x=166  y=70   width=15   height=32   xoffset=-4   yoffset=8    xadvance=11   page=0    chnl=0 
char id=109   x=393  y=102  width=37   height=26   xoffset=-4   yoffset=14   xadvance=32   page=0    chnl=0 
char id=110   x=430  y=102  width=25   height=26   xoffset=-4   yoffset=14   xadvance=21   page=0    chnl=0 
char id=111   x=455  y=102  width=27   height=26   xoffset=-4   yoffset=14   xadvance=21   page=0    chnl=0 
char id=112   x=181  y=70   width=26   height=32   xoffset=-4   yoffset=14   xadvance=21   page=0    chnl=0 
char id=113   x=207  y=70   width=26   height=32   xoffset=-4   yoffset=14   xadvance=21   page=0    chnl=0 
char id=114   x=482  y=102  width=22   height=26   xoffset=-4   yoffset=14   xadvance=14   page=0    chnl=0 
char id=115   x=0    y=134  width=27   height=26   xoffset=-4   yoffset=14   xadvance=20   page=0    chnl=0 
char id=116   x=216  y=102  width=21   height=31   xoffset=-4   yoffset=9    xadvance=14   page=0    chnl=0 
char id=117   x=27   y=134  width=25   height=26   xoffset=-4   yoffset=14   xadvance=21   page=0    chnl=0 
char id=118   x=52   y=134  width=27   height=26   xoffset=-4   yoffset=14   xadvance=20   page=0    chnl=0 
char id=119   x=79   y=134  width=39   height=26   xoffset=-4   yoffset=14   xadvance=30   page=0    chnl=0 
char id=120   x=118  y=134  width=29   height=26   xoffset=-4   yoffset=14   xadvance=21   page=0    chnl=0 
char id=121   x=233  y=70   width=28   height=32   xoffset=-4   yoffset=14   xadvance=20   page=0    chnl=0 
char id=122   x=147  y=134  width=24   height=26   xoffset=-4   yoffset=14   xadvance=18   page=0    chnl=0 
char id=123   x=95   y=0    width=21   height=38   xoffset=-4   yoffset=8    xadvance=12   page=0    chnl=0 
char id=124   x=137  y=0    width=13   height=38   xoffset=-4   yoffset=8    xadvance=9    page=0    chnl=0 
char id=125   x=116  y=0    width=21   height=38   xoffset=-4   yoffset=8    xadvance=12   page=0    chnl=0 
char id=126   x=337  y=134  width=26   height=17   xoffset=-4   yoffset=15   xadvance=21   page=0    chnl=0 
kernings count=241
kerning first=102 second=101 amount=-1
kerning first=76 second=89 amount=-2
kerning first=76 second=79 amount=-1
kerning first=80 second=65 amount=-3
kerning first=102 second=111 amount=-1
kerning first=86 second=97 amount=-1
kerning first=84 second=114 amount=-1
kerning first=119 second=44 amount=-2
kerning first=65 second=79 amount=-1
kerning first=86 second=67 amount=-1
kerning first=66 second=44 amount=1
kerning first=84 second=45 amount=-2
kerning first=80 second=46 amount=-6
kerning first=65 second=89 amount=-3
kerning first=67 second=46 amount=1
kerning first=112 second=119 amount=-1
kerning first=84 second=67 amount=-1
kerning first=84 second=58 amount=-1
kerning first=84 second=59 amount=-1
kerning first=114 second=113 amount=1
kerning first=102 second=63 amount=3
kerning first=80 second=111 amount=-1
kerning first=86 second=114 amount=-1
kerning first=89 second=67 amount=-2
kerning first=114 second=111 amount=1
kerning first=88 second=71 amount=-1
kerning first=80 second=101 amount=-1
kerning first=87 second=46 amount=-2
kerning first=75 second=117 amount=-1
kerning first=114 second=100 amount=1
kerning first=86 second=117 amount=-1
kerning first=87 second=79 amount=-1
kerning first=86 second=79 amount=-1
kerning first=81 second=86 amount=-1
kerning first=82 second=81 amount=-1
kerning first=115 second=119 amount=-1
kerning first=65 second=85 amount=-1
kerning first=68 second=87 amount=1
kerning first=112 second=46 amount=-1
kerning first=81 second=89 amount=-2
kerning first=114 second=97 amount=1
kerning first=84 second=97 amount=-2
kerning first=65 second=86 amount=-2
kerning first=84 second=99 amount=-2
kerning first=89 second=100 amount=-3
kerning first=102 second=102 amount=1
kerning first=114 second=103 amount=1
kerning first=87 second=104 amount=1
kerning first=86 second=105 amount=1
kerning first=114 second=106 amount=1
kerning first=74 second=111 amount=-1
kerning first=99 second=108 amount=-1
kerning first=84 second=109 amount=-1
kerning first=114 second=110 amount=2
kerning first=118 second=111 amount=-1
kerning first=89 second=112 amount=-2
kerning first=70 second=114 amount=-1
kerning first=97 second=116 amount=-1
kerning first=89 second=117 amount=-2
kerning first=76 second=119 amount=-1
kerning first=101 second=120 amount=-1
kerning first=99 second=121 amount=-1
kerning first=82 second=119 amount=1
kerning first=66 second=65 amount=-1
kerning first=81 second=84 amount=-1
kerning first=80 second=44 amount=-6
kerning first=97 second=103 amount=1
kerning first=111 second=120 amount=-1
kerning first=65 second=112 amount=1
kerning first=84 second=46 amount=-5
kerning first=86 second=101 amount=-2
kerning first=119 second=104 amount=1
kerning first=114 second=117 amount=1
kerning first=121 second=44 amount=-3
kerning first=121 second=46 amount=-3
kerning first=89 second=97 amount=-3
kerning first=75 second=101 amount=-1
kerning first=65 second=118 amount=-1
kerning first=70 second=97 amount=-1
kerning first=119 second=113 amount=-1
kerning first=75 second=111 amount=-1
kerning first=75 second=71 amount=-1
kerning first=87 second=97 amount=-1
kerning first=101 second=103 amount=1
kerning first=87 second=105 amount=1
kerning first=81 second=46 amount=1
kerning first=86 second=71 amount=-1
kerning first=67 second=44 amount=1
kerning first=114 second=99 amount=1
kerning first=70 second=44 amount=-5
kerning first=66 second=85 amount=-1
kerning first=76 second=86 amount=-2
kerning first=89 second=59 amount=-2
kerning first=84 second=44 amount=-5
kerning first=120 second=101 amount=-1
kerning first=108 second=121 amount=1
kerning first=89 second=101 amount=-3
kerning first=114 second=46 amount=-2
kerning first=121 second=99 amount=-1
kerning first=114 second=101 amount=1
kerning first=101 second=118 amount=-1
kerning first=81 second=44 amount=1
kerning first=74 second=121 amount=-1
kerning first=82 second=79 amount=-1
kerning first=89 second=113 amount=-3
kerning first=119 second=46 amount=-3
kerning first=68 second=89 amount=-1
kerning first=119 second=111 amount=-1
kerning first=102 second=46 amount=-2
kerning first=86 second=44 amount=-4
kerning first=114 second=109 amount=2
kerning first=79 second=89 amount=-2
kerning first=86 second=65 amount=-2
kerning first=103 second=114 amount=1
kerning first=102 second=33 amount=3
kerning first=84 second=122 amount=-1
kerning first=86 second=46 amount=-4
kerning first=70 second=101 amount=-1
kerning first=99 second=104 amount=-1
kerning first=89 second=44 amount=-5
kerning first=68 second=86 amount=-1
kerning first=88 second=79 amount=-1
kerning first=82 second=89 amount=-1
kerning first=75 second=67 amount=-1
kerning first=98 second=118 amount=-1
kerning first=89 second=65 amount=-3
kerning first=114 second=107 amount=1
kerning first=84 second=101 amount=-2
kerning first=89 second=79 amount=-2
kerning first=114 second=118 amount=2
kerning first=84 second=111 amount=-2
kerning first=120 second=111 amount=-1
kerning first=82 second=111 amount=-1
kerning first=79 second=87 amount=-1
kerning first=84 second=71 amount=-1
kerning first=118 second=100 amount=-1
kerning first=74 second=97 amount=-1
kerning first=89 second=45 amount=-3
kerning first=84 second=81 amount=-1
kerning first=114 second=115 amount=1
kerning first=68 second=46 amount=-1
kerning first=79 second=44 amount=-1
kerning first=121 second=100 amount=-1
kerning first=114 second=112 amount=1
kerning first=87 second=71 amount=-1
kerning first=89 second=83 amount=-1
kerning first=86 second=111 amount=-2
kerning first=87 second=111 amount=-1
kerning first=70 second=46 amount=-5
kerning first=81 second=65 amount=1
kerning first=114 second=58 amount=1
kerning first=84 second=104 amount=1
kerning first=114 second=116 amount=2
kerning first=87 second=101 amount=-1
kerning first=65 second=67 amount=-1
kerning first=70 second=65 amount=-3
kerning first=65 second=81 amount=-1
kerning first=118 second=46 amount=-3
kerning first=103 second=121 amount=1
kerning first=87 second=67 amount=-1
kerning first=74 second=46 amount=-1
kerning first=76 second=67 amount=-1
kerning first=75 second=79 amount=-1
kerning first=87 second=44 amount=-1
kerning first=89 second=58 amount=-2
kerning first=86 second=59 amount=-1
kerning first=114 second=121 amount=2
kerning first=121 second=97 amount=-1
kerning first=76 second=87 amount=-1
kerning first=118 second=44 amount=-3
kerning first=79 second=84 amount=-1
kerning first=111 second=119 amount=-1
kerning first=111 second=118 amount=-1
kerning first=84 second=108 amount=1
kerning first=82 second=117 amount=-1
kerning first=89 second=46 amount=-5
kerning first=111 second=122 amount=1
kerning first=84 second=65 amount=-2
kerning first=74 second=117 amount=-1
kerning first=107 second=111 amount=-1
kerning first=85 second=65 amount=-1
kerning first=111 second=121 amount=-1
kerning first=114 second=59 amount=1
kerning first=65 second=84 amount=-2
kerning first=89 second=118 amount=-1
kerning first=86 second=58 amount=-1
kerning first=87 second=100 amount=-1
kerning first=76 second=84 amount=-2
kerning first=102 second=32 amount=2
kerning first=111 second=46 amount=-1
kerning first=114 second=114 amount=1
kerning first=82 second=86 amount=-1
kerning first=74 second=44 amount=-1
kerning first=118 second=99 amount=-1
kerning first=86 second=81 amount=-1
kerning first=79 second=88 amount=-2
kerning first=85 second=44 amount=-1
kerning first=118 second=101 amount=-1
kerning first=88 second=67 amount=-1
kerning first=79 second=86 amount=-1
kerning first=75 second=118 amount=-1
kerning first=76 second=85 amount=-1
kerning first=107 second=101 amount=-1
kerning first=84 second=117 amount=-1
kerning first=120 second=99 amount=-1
kerning first=75 second=121 amount=-1
kerning first=84 second=115 amount=-2
kerning first=114 second=44 amount=-2
kerning first=82 second=71 amount=-1
kerning first=85 second=46 amount=-1
kerning first=70 second=111 amount=-1
kerning first=97 second=118 amount=-1
kerning first=121 second=111 amount=-1
kerning first=79 second=65 amount=-1
kerning first=82 second=101 amount=-1
kerning first=65 second=71 amount=-1
kerning first=102 second=44 amount=-2
kerning first=112 second=121 amount=-1
kerning first=74 second=101 amount=-1
kerning first=86 second=45 amount=-1
kerning first=121 second=101 amount=-1
kerning first=84 second=79 amount=-1
kerning first=89 second=111 amount=-3
kerning first=118 second=113 amount=-1
kerning first=74 second=65 amount=-1
kerning first=76 second=71 amount=-1
kerning first=79 second=46 amount=-1
kerning first=68 second=65 amount=-1
kerning first=114 second=108 amount=1
kerning first=71 second=44 amount=1
kerning first=110 second=118 amount=-1
kerning first=82 second=85 amount=-1
kerning first=87 second=117 amount=-1
kerning first=120 second=100 amount=-1
kerning first=80 second=97 amount=-1
kerning first=99 second=107 amount=-1
kerning first=66 second=46 amount=1
kerning first=89 second=71 amount=-2
kerning first=107 second=121 amount=1
kerning first=82 second=67 amount=-1
kerning first=84 second=121 amount=-1"#;
