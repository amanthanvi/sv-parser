# Build Stage
FROM ghcr.io/amanthanvi/cargo-fuzz as builder

## Add source code to the build stage.
ADD . /
WORKDIR /

RUN echo building instrumented harnesses && \
    bash -c "pushd crates/sv-parser/fuzz && cargo +nightly -Z sparse-registry fuzz build && popd" && \
    mv crates/sv-parser/fuzz/target/x86_64-unknown-linux-gnu/release/lexer /lexer && \
    mv crates/sv-parser/fuzz/target/x86_64-unknown-linux-gnu/release/lite_parser /lite_parser && \
    echo done

RUN echo building non-instrumented harnesses && \
    export RUSTFLAGS="--cfg fuzzing -Clink-dead-code -Cdebug-assertions -C codegen-units=1" && \
    bash -c "pushd crates/sv-parser/fuzz && cargo +nightly -Z sparse-registry build --release && popd" && \
    mv crates/sv-parser/fuzz/target/release/lexer /lexer_no_inst && \
    mv crates/sv-parser/fuzz/target/release/lite_parser /lite_parser_no_inst && \
    echo done

# Package Stage
FROM rustlang/rust:nightly

COPY --from=builder /lexer /lexer_no_inst /
COPY --from=builder /lite_parser /lite_parser_no_inst /
