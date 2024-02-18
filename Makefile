NAME := n_puzzle

release:
	@cd n-puzzle && cargo build --release
	@cp n-puzzle/target/release/n-puzzle ./$(NAME)

debug:
	@cd n-puzzle && cargo build
	@cp n-puzzle/target/debug/n-puzzle ./$(NAME)

test:
	@cd n-puzzle && cargo test

$(NAME): release

all: release

clean:
	@cd n-puzzle && cargo clean

fclean: clean
	@rm -f ./$(NAME)

re: fclean all

.PHONY: all clean fclean re release debug test
