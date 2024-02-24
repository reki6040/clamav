# SPDX-License-Identifier: GPL-2.0-or-later
#
# Copyright (C) 2020 Olliver Schinagl <oliver@schinagl.nl>
# Copyright (C) 2021-2023 Cisco Systems, Inc. and/or its affiliates. All rights reserved.

#Memo: AS builder������build�݂̂ŋN�����͗��p���Ȃ��R���e�i�ƂȂ�悤���B

FROM index.docker.io/library/debian:11-slim AS builder

WORKDIR /src

COPY . /src/

ENV DEBIAN_FRONTEND noninteractive
ENV CARGO_HOME /src/build

RUN apt update && apt install -y \
        cmake \
        bison \
        flex \
        gcc \
        git \
        make \
        man-db \
        net-tools \
        pkg-config \
        python3 \
        python3-pip \
        python3-pytest \
        check \
        libbz2-dev \
        libcurl4-openssl-dev \
        libjson-c-dev \
        libncurses-dev \
        libpcre2-dev \
        libssl-dev \
        libxml2-dev \
        zlib1g-dev \
        curl \
    && \
    rm -rf /var/cache/apt/archives \
    && \
    # Using rustup to install Rust rather than rust:1.62.1-bullseye, because there is no rust:1.62.1-bullseye image for ppc64le at this time.
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    && \
    . $CARGO_HOME/env \
    && \
    rustup update \
    && \
    mkdir -p "./build" && cd "./build" \
    && \
    cmake .. \
          -DCARGO_HOME=$CARGO_HOME \
          -DCMAKE_BUILD_TYPE="Release" \
          -DCMAKE_INSTALL_PREFIX="/usr" \
          -DCMAKE_INSTALL_LIBDIR="/usr/lib" \
          -DAPP_CONFIG_DIRECTORY="/etc/clamav" \
          -DDATABASE_DIRECTORY="/var/lib/clamav" \
          -DENABLE_CLAMONACC=ON\
          -DENABLE_EXAMPLES=OFF \
          -DENABLE_JSON_SHARED=ON \
          -DENABLE_MAN_PAGES=OFF \
          -DENABLE_MILTER=OFF \
          -DENABLE_STATIC_LIB=OFF \
    && \
    make DESTDIR="/clamav" -j$(($(nproc) - 1)) install \
    && \
    rm -r \
       "/clamav/usr/include" \
       "/clamav/usr/lib/pkgconfig/" \
    && \
    sed -e "s|^\(Example\)|\# \1|" \
        -e "s|.*\(PidFile\) .*|\1 /tmp/clamd.pid|" \
        -e "s|.*\(LocalSocket\) .*|\1 /tmp/clamd.sock|" \
        -e "s|.*\(TCPSocket\) .*|\1 3310|" \
        -e "s|.*\(TCPAddr\) .*|#\1 0.0.0.0|" \
        -e "s|.*\(User\) .*|\1 clamav|" \
        -e "s|^\#\(LogFile\) .*|\1 /var/log/clamav/clamd.log|" \
        -e "s|^\#\(LogTime\).*|\1 yes|" \
        "/clamav/etc/clamav/clamd.conf.sample" > "/clamav/etc/clamav/clamd.conf" && \
    sed -e "s|^\(Example\)|\# \1|" \
        -e "s|.*\(PidFile\) .*|\1 /tmp/freshclam.pid|" \
        -e "s|.*\(DatabaseOwner\) .*|\1 clamav|" \
        -e "s|^\#\(UpdateLogFile\) .*|\1 /var/log/clamav/freshclam.log|" \
        -e "s|^\#\(NotifyClamd\).*|\1 /etc/clamav/clamd.conf|" \
        -e "s|^\#\(ScriptedUpdates\).*|\1 yes|" \
        "/clamav/etc/clamav/freshclam.conf.sample" > "/clamav/etc/clamav/freshclam.conf" || \
    exit 1 \
    && \
    ctest -V && \
    cd / && \
    tar zcf clamav.tgz clamav && \
    mv clamav.tgz /clamav

FROM index.docker.io/library/debian:11-slim

LABEL maintainer="ClamAV bugs <clamav-bugs@external.cisco.com>"

EXPOSE 3310

ENV DEBIAN_FRONTEND=noninteractive
#ENV TZ Etc/UTC
ENV TZ Asia/Tokyo

RUN apt-get update && apt-get install -y \
        libbz2-1.0 \
        libcurl4 \
        libssl1.1 \
        libjson-c5 \
        libncurses6 \
        libpcre2-8-0 \
        libxml2 \
        zlib1g \
        tzdata \
        netcat \
    && \
    rm -rf /var/cache/apt/archives && \
    groupadd -g 1000 "clamav" && \
    useradd -m -g clamav -s /bin/false --home-dir /var/lib/clamav -u 1000 -c "Clam Antivirus" clamav && \
    install -d -m 755 -g "clamav" -o "clamav" "/var/log/clamav" && \
    chown -R clamav:clamav /var/lib/clamav

COPY --from=builder "/clamav" "/"

COPY "./scripts/clamdcheck.sh" "/usr/local/bin/"
COPY "./scripts/docker-entrypoint.sh" "/init"
COPY "./scripts/docker-entrypoint-unprivileged.sh" "/init-unprivileged"

HEALTHCHECK --start-period=6m CMD clamdcheck.sh

ENTRYPOINT [ "/init" ]
