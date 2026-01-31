package main

import (
	"fmt";
	"os";
	"strconv";
)

func main() {
	file, err := os.ReadFile("input.txt");
	if err != nil {
		fmt.Println("Error in reading file", err);
	}

	strData := string(file);
	vec2d := make([][]int, 0);
	l := 0;

	for i := 0; i < len(strData); i++ {
		if strData[i] == '\n' {
			intVec, err := strconv.ParseInt(strData[l:i], 10, 64);
			if err != nil {
				fmt.Println("Unable to Parse String");
			}

			vec2d = append(vec2d, []int{[]int{int(intVec)}...});
			l = i+1;
		}
		
	}
	fmt.Println(vec2d);
}
