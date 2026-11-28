# TEST

## Extension

![png](test1.png)
![jpg](test2.jpg)
![gif](test3.gif)
![webp](test4.webp)
![avif](test5.avif)

## Directory

![dir](dir-test/test6.webp)

## Extension 2

![jxl](test7.jxl)

## Not found

![not found](test11.png)

## Side by side

![png](test1.png)![jpg](test2.jpg)
![png](test1.png) ![jpg](test2.jpg)
![not found](test11.png)![jpg](test2.jpg)

## Table

|Header Left|Header Right|
|:---:|:---:|
|![left](test4.webp)|![right](dir-test/test6.webp)|

|Header Left|Header Center|Header Right|
|:---:|:---:|:---:|
|![left](test4.webp)|![gif](test3.gif)|![right](dir-test/test6.webp)|

## alt

![avif](test5.avif)
![エーブイアイえふ](test5.avif)

## Image metadata

![caption](test1.png 'A "quote" & <tag>')
![empty title](test1.png "")
![before ![middle ![inner](test3.gif) after inner](test2.jpg) after middle](test1.png)![sibling](test2.jpg)
![unmeasured ![inner](test3.gif)](test11.png)![sibling](test2.jpg)
![inline $x < y$ and display $$a > b & c$$](test1.png)

## media-slider

```media-slider aria-label="アルバムの画像"
![avif](test5.avif)
![エーブイアイえふ](test5.avif)
```

```media-slider aria-label="slider-space1"
 ![avif](test5.avif)
 ![エーブイアイえふ](test5.avif)
```

```media-slider aria-label="slider-space2"
  ![avif](test5.avif)
  ![エーブイアイえふ](test5.avif)
```

```media-slider aria-label="slider-space3"
   ![avif](test5.avif)
   ![エーブイアイえふ](test5.avif)
```

```media-slider aria-label="slider-space4 (Since it is treated as a code block, it is not converted)"
    ![avif](test5.avif)
    ![エーブイアイえふ](test5.avif)
```

```media-slider aria-label="example"
![avif](test5.avif)
<div>
  ![エーブイアイえふ](test5.avif)
</div>
```

```media-slider aria-label="example"
![avif](test5.avif)
<div>
  テスト
  ![エーブイアイえふ](test5.avif)
</div>
```

```media-slider aria-label="example"
![avif](test5.avif)
<div>
 テスト1
 ![エーブイアイえふ](test5.avif)
 <div>
  テスト2
  ![エーブイアイえふ](test5.avif)
 </div>
</div>
```

```media-slider aria-label="example2"
![avif](test5.avif)
<div class="ft-definition">
 ![エーブイアイえふ](test5.avif)
 テスト
</div>
```

# Admonish

```admonish info
![avif](test5.avif)
![エーブイアイえふ](test5.avif)
```

~~~admonish info
```media-slider aria-label="admonish-slider-1"
![avif](test5.avif)
![エーブイアイえふ](test5.avif)
```
~~~

````admonish info
```media-slider aria-label="admonish-slider-2"
![avif](test5.avif)
![エーブイアイえふ](test5.avif)
```
````

~~~admonish info title="adomonish-abc"
```media-slider aria-label="adomonish-slider-title"
![avif](test5.avif)
![エーブイアイえふ](test5.avif)
```
~~~

# Span

<span>

<img src="test5.avif" alt="avif" width="59" height="44" loading="lazy">
<img src="test5.avif" alt="エーブイアイえふ" width="59" height="44" loading="lazy">

</span>

<div>

<img src="test5.avif" alt="avif" width="59" height="44" loading="lazy">
<img src="test5.avif" alt="エーブイアイえふ" width="59" height="44" loading="lazy">

</div>
