#! /usr/bin/env bash

images=(
    k8s.gcr.io/kube-proxy:v1.16.5,gotok8s/kube-proxy:v1.18.8
    k8s.gcr.io/kube-controller-manager:v1.16.5,gotok8s/kube-controller-manager:v1.18.8
    k8s.gcr.io/kube-scheduler:v1.16.5,gotok8s/kube-scheduler:v1.18.8
    k8s.gcr.io/kube-apiserver:v1.16.5,gotok8s/kube-apiserver:v1.18.8
    k8s.gcr.io/coredns:1.6.2,gotok8s/coredns:1.6.2
    k8s.gcr.io/pause:3.1,gotok8s/pause:3.1
    k8s.gcr.io/etcd:3.3.15-0,gotok8s/etcd:3.3.15-0
)

echo "start install docker images"

for image in ${images[@]}; do
    image=(${image//,/ })
    key=${image[0]}
    value=${image[1]}
    docker pull ${value}
    docker tag ${value} ${key}
    docker rmi ${value}
done

echo "finished install docker images"
