package scrabble

import (
    
"strings"
)

func Score(word string) int {
   
word = strings.ToLower(word)
    sum := 0

    for i := 0; i < len(word); i++{
        switch word[i] {
            case 'a','e','i','o','u','l','n','r','s','t':
            sum+=1
            case 'd','g':
            sum+=2
            case 'b','c','m','p':
            sum+=3
            case 'f','h','v','w','y':
            sum+=4
            case 'k':
            sum+=5
            case 'j','x':
            sum+=8
            case 'q','z':
            sum+=10
            default:
            sum+=0
        }
    }
	return sum
}
