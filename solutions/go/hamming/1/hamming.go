package hamming
import ("fmt")
func Distance(a, b string) (int, error) {
    sum:=0
	if len(a) != len(b) {
        return 0, fmt.Errorf("invalid lenght")
    }

    for i:=0;i< len(a);i++{
        if a[i] != b[i] {
            sum++
        }
    }
    return sum, nil
}
