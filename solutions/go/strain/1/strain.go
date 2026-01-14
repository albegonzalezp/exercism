package strain

func Keep[T any] (collection []T, predicate func(T) bool) []T{
	result := make([]T, 0)
    
    for _, item := range collection {
        if predicate(item){
            result = append(result, item)
        }
    }

    return result
}

func Discard[T any] (collection []T, predicate func(T) bool) []T{
	result := make([]T, 0)
    
    for _, item := range collection {
        if !predicate(item){
            result = append(result, item)
        }
    }

    return result
}