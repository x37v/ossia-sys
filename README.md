# ossia-sys

rust 'sys' bindings for [https://github.com/OSSIA/libossia](libossia).

[docs](http://ossia.github.io/libossia/html/group___c_a_p_i.html) for the bound API.

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

