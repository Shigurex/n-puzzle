#include "Board.hpp"

Board::Board(size_t size, BoardType board)
{
	this->_size = size;
	std::set<size_t> inputs;

	if (board.size() != size)
		throw InvalidBoardFormatException();
	for (size_t i = 0; i < size; i++) {
		if (board[i].size() != size)
			throw InvalidBoardFormatException();
		for (size_t j = 0; j < size; j++) {
			if (board[i][j] == 0)
				this->_empty_pos = {i, j};
			inputs.insert(board[i][j]);
		}
	}
	if (inputs.size() != size * size
		|| *inputs.begin() != 0
		|| *inputs.rbegin() != size * size - 1)
		throw InvalidBoardFormatException();
	this->_board = board;
}

Board::~Board(void) {}

size_t Board::getValue(Position pos) const
{
	if (pos.row >= this->_size || pos.col >= this->_size)
		throw InvalidOperation();
	return (this->_board[pos.row][pos.col]);
}

bool Board::isSolved(void) const
{
	size_t value = 1;
	size_t num_boxes = this->_size * this->_size;

	for (size_t i = 0; i < this->_size; i++) {
		for (size_t j = 0; j < this->_size; j++) {
			if (this->_board[i][j] != value % num_boxes)
				return (false);
			value++;
		}
	}
	return (true);
}

const char *Board::InvalidBoardFormatException::what() const throw()
{
	return ("Invalid board format");
}

const char *Board::InvalidOperation::what() const throw()
{
	return ("Invalid operation");
}

