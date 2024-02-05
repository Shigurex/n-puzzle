#ifndef BOARD_HPP
# define BOARD_HPP

# include <vector>
# include <set>
# include <iostream>
# include <exception>

typedef std::vector<std::vector<size_t> > BoardType;

typedef struct s_position
{
	size_t row;
	size_t col;
} Position;

class Board
{
	private:
		size_t _size;
		BoardType _board;
		Position _empty_pos;

	public:
		Board(size_t size, std::vector<std::vector<size_t> > board);
		~Board(void);

		size_t getValue(Position pos) const;
		bool isSolved(void) const;

		class InvalidBoardFormatException : public std::exception
		{
			public:
				virtual const char *what() const throw();
		};

		class InvalidOperation : public std::exception
		{
			public:
				virtual const char *what() const throw();
		};
};

#endif
