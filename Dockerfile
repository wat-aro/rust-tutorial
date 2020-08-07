FROM rust:1.45.2

ENV TZ=Asia/Tokyo \
    APP_HOME=/src

ENV PATH $PATH:/root/.local/bin

WORKDIR $APP_HOME
