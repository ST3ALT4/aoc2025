package main

import (
	"bufio"
	"fmt"
	"os"
)

func maxJolt(bat []int) (int, int) {

	firstMax := -1
	secondMax := -1

	for _, value := range bat {
		if value >= firstMax {
			firstMax = value
			secondMax = value
		}
	}

	return secondMax, firstMax
}

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		fmt.Println("Error in reading file", err)
	}

	defer file.Close()

	scanner := bufio.NewScanner(file)

	for scanner.Scan() {

		line := scanner.Text()
		fmt.Println(line)
	}

}
