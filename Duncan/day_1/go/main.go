package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"sort"
	"strconv"
	"strings"
)

func getSlicesFromInput() [2][]int {
	file, err := os.Open("input")
	if err != nil {
		panic(err)
	}

	defer file.Close()

	left := []int{}
	right := []int{}

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		split := strings.Fields(scanner.Text())

		leftVal, lErr := strconv.Atoi(split[0])
		rightVal, rErr := strconv.Atoi(split[1])

		if lErr != nil {
			panic(lErr)
		}

		if rErr != nil {
			panic(rErr)
		}

		left = append(left, leftVal)
		right = append(right, rightVal)
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	sort.Sort(sort.IntSlice(left))
	sort.Sort(sort.IntSlice(right))

	return [2][]int{left, right}
}

func partOne(left []int, right []int) {
	result := 0

	for i := 0; i < len(left); i++ {
		leftVal := left[i]
		rightVal := right[i]

		value := 0

		if leftVal > rightVal {
			value = leftVal - rightVal
		} else {
			value = rightVal - leftVal
		}

		result = result + value
	}

	fmt.Printf("Part one result: %d\n", result)
}

func partTwo(left []int, right []int) {
	result := 0

	for i := 0; i < len(left); i++ {
        leftVal := left[i]
        rightCount := 0

        foundRight := false

        inner: for j := 0; j < len(right); j++ {
            rightVal := right[j]
            if rightVal == leftVal {
                rightCount += 1

                if !foundRight {
                    foundRight = true
                }
            } else {
                if foundRight {
                    break inner
                }
            }
        }

        result += rightCount * leftVal
    }

    fmt.Printf("Part two result: %d\n", result)
}

func main() {
	slices := getSlicesFromInput()
	left := slices[0]
	right := slices[1]

	partOne(left, right)
    partTwo(left, right)
}
