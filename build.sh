if [ $1 = "release" ]; then
    arch=`uname -m`

    if [ $arch = "arm64" ]; then
        arch="aarch64"
    fi

    os=`uname`

    if [ $os = "Linux" ]; then
        os="linux"
    elif [ $os = "Darwin" ]; then
        os="darwin"
    else
        echo ERR
        exit 1
    fi


    dir=cck-`git rev-parse --abbrev-ref @`-$os-$arch

    cargo build --features=alloc --release && \
    mkdir $dir && cp -r ./target/release/* $dir/ && cp ./LICENSE $dir/LICENSE && cp ./README.md $dir/README.md && cp ./image.png $dir/image.png && \
    tar -zcvf $dir.tar.gz ./$dir && \
    rm -r ./$dir
else
    cargo build --features=alloc
fi