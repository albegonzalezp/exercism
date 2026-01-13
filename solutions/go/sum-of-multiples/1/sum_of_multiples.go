package summultiples



func SumMultiples(limit int, divisors ...int) int {
    visited := make(map[int]bool)
    result := 0

        for _, div := range divisors {
            if div == 0 {
                continue
            }
            
            for i := 1; i < limit; i++{
                if i % div == 0 {
                    if visited[i] {
                        continue
                    }
                    result+=i
                    visited[i] = true
                }
            }
        }    
    return result
}
