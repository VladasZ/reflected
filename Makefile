
lint:
	cargo clippy \
        -- \
        \
        -W clippy::all \
        -W clippy::pedantic \
        \
        -A clippy::must-use-candidate \
        -A clippy::return-self-not-must-use \
        -A clippy::missing-errors-doc \
        -A clippy::needless-pass-by-value \
        -A clippy::module-name-repetitions \
        -A clippy::missing_panics_doc \
        \
        -D warnings

test:
	cargo test --all
	cargo test --all --release
