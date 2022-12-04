# Build Stage
FROM ghcr.io/amanthanvi/cargo-fuzz as builder

## Add source code to the build stage.
ADD . /
WORKDIR /

RUN echo building instrumented harnesses && \
    bash -c "pushd /sv-parser/sv-parser/fuzz && cargo +nightly -Z sparse-registry fuzz build && popd" && \
    mv /sv-parser/sv-parser/fuzz/target/x86_64-unknown-linux-gnu/release/lexer /lib && \

RUN echo building non-instrumented harnesses && \
    export RUSTFLAGS="--cfg fuzzing -Clink-dead-code -Cdebug-assertions -C codegen-units=1" && \
    bash -c "pushd /sv-parser/sv-parser/fuzz && cargo +nightly -Z sparse-registry build --release && popd" && \
    mv /sv-parser/sv-parser/fuzz/target/release/lib /lib_no_inst && \
    echo done

# Package Stage
FROM rustlang/rust:nightly

COPY --from=builder /lib /lib_no_inst /
