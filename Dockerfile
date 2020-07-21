FROM rust:1.45.0

ENV TZ=Asia/Tokyo \
    APP_HOME=/src

ENV PATH $PATH:/root/.local/bin

WORKDIR $APP_HOME
