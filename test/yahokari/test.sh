PYTHON=python
NPUZZLE_PATH=npuzzle-gen.py

make
echo "Error: No solution" > output.error.txt
for i in {1..10}
do
	${PYTHON} ${NPUZZLE_PATH} 3 -u > input.txt && ./n_puzzle ./input.txt &> output.txt
	diff output.error.txt output.txt > /dev/null
	if [ $? -eq 0 ]; then
		echo "Test $i: OK"
	else
		echo "Test $i: KO"
	fi

	rm input.txt output.txt
done

for i in {1..10}
do
	${PYTHON} ${NPUZZLE_PATH} 3 -s > input.txt && ./n_puzzle ./input.txt &> output.txt
	diff output.error.txt output.txt > /dev/null
	if [ $? -ne 0 ]; then
		echo "Test $i: OK"
	else
		echo "Test $i: KO"
	fi

	rm input.txt output.txt
done

rm output.error.txt
