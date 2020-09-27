# logcheck
A unique amount of ip addresses counter for web server access files

I created this because, regular shell(grep -o '[0-9]\{1,3\}\.[0-9]\{1,3\}\.[0-9]\{1,3\}\.[0-9]\{1,3\}' /var/log/hiawatha/blogaccess.log|uniq| wc -l) is slow a shell for large files

## Install rust on openbsd
```
pkg_add -iv rust
```


## install me
```shell
$ cargo build
```

## run me 
```shell
./logcheck /var/log/mysite_access.log.4
logcheck by flipchan
Amount of unique ips in the file is 43

```

Use the openbsd version for [Pledge](https://man.openbsd.org/pledge) support


## Special thanks to 
@stephaneyfx !
