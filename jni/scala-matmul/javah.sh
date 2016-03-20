#! /bin/bash

export XCP="${HOME}/.sbt/boot/scala-2.10.4/lib/scala-library.jar:${HOME}/.sbt/boot/scala-2.10.4/lib/scala-reflect.jar:${PWD}/target/scala-2.10/classes"
javah -cp ${XCP} MatMul$
