## building

make sure to initialize the submodule
```
pushd deps/libossia/
git submodule init
git submodule update
popd
```

I had to set CC and CXX to clang to build

```
CC=/usr/bin/clang CXX=/usr/bin/clang++ cargo build
```
