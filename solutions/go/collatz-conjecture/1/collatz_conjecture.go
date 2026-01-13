package collatzconjecture

import (
    "fmt"
)

func CollatzConjecture(n int) (int, error) {
	steps := 0

    if n < 1 {
        return 0, fmt.Errorf("invalid")
    }
    
    for {
        if n == 1 {
            return steps, nil
        }
        
        if n % 2 == 0 {
            n /= 2
            steps++
        } else {
            n = n * 3 + 1
            steps++
        }
    }
}
