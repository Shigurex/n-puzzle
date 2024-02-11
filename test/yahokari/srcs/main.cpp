#include <Board.hpp>

int main(void)
{
	Board board(4, {{1, 2, 3, 4}, {5, 6, 7, 8}, {9, 10, 11, 12}, {13, 14, 15, 0}});

	std::cout << "Board is solved: " << (board.isSolved() ? "TRUE" : "FALSE") << std::endl;
	return (0);
}
