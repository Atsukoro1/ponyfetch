#!/bin/bash

function makeDirectories() {
    echo "Creating required directories..."
    
    if [ ! -d "/usr/share/ponyfetch" ]; then
        mkdir /usr/share/ponyfetch
        mkdir /usr/share/ponyfetch/ponies
    fi
}

function compile() {
    echo "Compiling ponyfetch..."
    
    if [ ! -f "/usr/bin/ponyfetch" ]; then
        cargo build --release
    fi
}

function moveFiles() {
    echo "Moving files..."
    
    if [ ! -f "/usr/bin/ponyfetch" ]; then
        cp target/release/ponyfetch /usr/bin/ponyfetch
    fi

    toCopyCount=$(ls -1 /usr/share/ponyfetch/ponies/*.txt 2>/dev/null | wc -l)
    dirCount=$(ls -1 ponies/*.txt 2>/dev/null | wc -l)

    if [ $toCopyCount != $dirCount ]; then
        cp -r ponies/* /usr/share/ponyfetch/ponies
    fi
}

echo "Thanks for choosing ponyfetch!"
echo "Let's begin installing!"

makeDirectories
compile
moveFiles