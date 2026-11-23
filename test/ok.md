# TEST

## Extension
<img src="test1.png" alt="png" width="50" height="28" fetchpriority="high">
<img src="test2.jpg" alt="jpg" width="59" height="34" loading="lazy">
<img src="test3.gif" alt="gif" width="59" height="44" loading="lazy">
<img src="test4.webp" alt="webp" width="59" height="44" loading="lazy">
<img src="test5.avif" alt="avif" width="59" height="44" loading="lazy">

## Directory
<img src="dir-test/test6.webp" alt="dir" width="59" height="44" loading="lazy">

## Extension 2
<img src="test7.jxl" alt="jxl" width="50" height="28" loading="lazy">

## Not found
![not found](test11.png)

## Side by side
<img src="test1.png" alt="png" width="50" height="28" loading="lazy"><img src="test2.jpg" alt="jpg" width="59" height="34" loading="lazy">
<img src="test1.png" alt="png" width="50" height="28" loading="lazy"> <img src="test2.jpg" alt="jpg" width="59" height="34" loading="lazy">
![not found](test11.png)<img src="test2.jpg" alt="jpg" width="59" height="34" loading="lazy">

## Table
|Header Left|Header Right|
|:---:|:---:|
|<img src="test4.webp" alt="left" width="59" height="44" loading="lazy">|<img src="dir-test/test6.webp" alt="right" width="59" height="44" loading="lazy">|

|Header Left|Header Center|Header Right|
|:---:|:---:|:---:|
|<img src="test4.webp" alt="left" width="59" height="44" loading="lazy">|<img src="test3.gif" alt="gif" width="59" height="44" loading="lazy">|<img src="dir-test/test6.webp" alt="right" width="59" height="44" loading="lazy">|

## alt
<img src="test5.avif" alt="avif" width="59" height="44" loading="lazy">
<img src="test5.avif" alt="エーブイアイえふ" width="59" height="44" loading="lazy">

## Image metadata
<img src="test1.png" alt="caption" title="A &quot;quote&quot; &amp; &lt;tag&gt;" width="50" height="28" loading="lazy">
<img src="test1.png" alt="empty title" width="50" height="28" loading="lazy">
<img src="test1.png" alt="before middle inner after inner after middle" width="50" height="28" loading="lazy"><img src="test2.jpg" alt="sibling" width="59" height="34" loading="lazy">
![unmeasured ![inner](test3.gif)](test11.png)<img src="test2.jpg" alt="sibling" width="59" height="34" loading="lazy">
<img src="test1.png" alt="inline $x &lt; y$ and display $$a &gt; b &amp; c$$" width="50" height="28" loading="lazy">

## media-slider
<media-slider role="group" aria-roledescription="carousel" aria-label="アルバムの画像">
<img src="test5.avif" alt="avif" width="59" height="44" loading="lazy">
<img src="test5.avif" alt="エーブイアイえふ" width="59" height="44" loading="lazy">
</media-slider>

<media-slider role="group" aria-roledescription="carousel" aria-label="slider-space1">
 <img src="test5.avif" alt="avif" width="59" height="44" loading="lazy">
 <img src="test5.avif" alt="エーブイアイえふ" width="59" height="44" loading="lazy">
</media-slider>

<media-slider role="group" aria-roledescription="carousel" aria-label="slider-space2">
  <img src="test5.avif" alt="avif" width="59" height="44" loading="lazy">
  <img src="test5.avif" alt="エーブイアイえふ" width="59" height="44" loading="lazy">
</media-slider>

<media-slider role="group" aria-roledescription="carousel" aria-label="slider-space3">
   <img src="test5.avif" alt="avif" width="59" height="44" loading="lazy">
   <img src="test5.avif" alt="エーブイアイえふ" width="59" height="44" loading="lazy">
</media-slider>

<media-slider role="group" aria-roledescription="carousel" aria-label="slider-space4 (Since it is treated as a code block, it is not converted)">
    ![avif](test5.avif)
    ![エーブイアイえふ](test5.avif)
</media-slider>

~~~admonish info
<media-slider role="group" aria-roledescription="carousel" aria-label="admonish-slider-1">
<img src="test5.avif" alt="avif" width="59" height="44" loading="lazy">
<img src="test5.avif" alt="エーブイアイえふ" width="59" height="44" loading="lazy">
</media-slider>
~~~

````admonish info
<media-slider role="group" aria-roledescription="carousel" aria-label="admonish-slider-2">
<img src="test5.avif" alt="avif" width="59" height="44" loading="lazy">
<img src="test5.avif" alt="エーブイアイえふ" width="59" height="44" loading="lazy">
</media-slider>
````

~~~admonish info title="adomonish-abc"
<media-slider role="group" aria-roledescription="carousel" aria-label="adomonish-slider-title">
<img src="test5.avif" alt="avif" width="59" height="44" loading="lazy">
<img src="test5.avif" alt="エーブイアイえふ" width="59" height="44" loading="lazy">
</media-slider>
~~~
