# logcheck
A unique amount of ip addresses counter for web server access files

I created this because, regular shell(grep -o '[0-9]\{1,3\}\.[0-9]\{1,3\}\.[0-9]\{1,3\}\.[0-9]\{1,3\}' /var/log/hiawatha/blogaccess.log|uniq| wc -l) is slow a shell for large files


## install me
```shell
$ cargo build
```

## run me 
```shell

```


## Special thanks to 
@stephaneyfx !
