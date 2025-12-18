for i in 1..25 {
    let folder = $"day($i)"
    mkdir $folder
    cd $folder
    cargo init
    cd ..
}
