
#echo $*
#echo ${@:2}

p=`echo $1 | sed 's|/*$||'`
#echo $p

mkdir $p
(
    cd $p

    mkdir template
    cp -r ../template/src/ template/
    cp ../template/Cargo.toml template/
    cp ../template/rust-toolchain.toml template/
    cp ../template/contests.md template/

    for i in ${@:2}
    do
        pp=$p"_"$i
        #echo $pp

        cargo new --edition 2024 $pp
        cp template/rust-toolchain.toml $pp/
        cp template/src/main.rs $pp/src/
        echo "---------- Created $pp"

        pps=$pps'{"path": "'$pp'"},'
        #echo $pps
    done

    pps=$pps'{"path": "template"},'

    #echo $pps
    cat ../template/template.code-workspace | sed "s/{.*path.*,/$pps/g" > $p.code-workspace
)

