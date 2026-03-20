
#echo $*
#echo ${@:2}

p=`echo $1 | sed 's|/*$||'`
#echo $p

mkdir $p
(
    cd $p

    for i in ${@:2}
    do
        pp=$p"_"$i
        #echo $pp

        cargo new --edition 2024 $pp
        cp ../template/rust-toolchain.toml $pp/
        cp ../template/src/main.rs $pp/src/
        echo "---------- Created $pp"

        pps=$pps'{"path": "'$pp'"},'
        #echo $pps
    done

    #echo $pps
    cat ../template/template.code-workspace | sed "s/{.*path.*,/$pps/g" > $p.code-workspace
)

