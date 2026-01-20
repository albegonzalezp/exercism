package protein

import "errors"

var (
    ErrStop        = errors.New("stop")
    ErrInvalidBase = errors.New("invalid base")
)

func FromRNA(rna string) ([]string, error) {
    result := []string{}

    for i := 0; i+3 <= len(rna); i += 3 {
        codon := rna[i : i+3]
        amino, err := FromCodon(codon)
        if err == ErrStop {
            break // stop translation at STOP codon
        } else if err != nil {
            return nil, err // return any invalid codon error
        }
        result = append(result, amino)
    }

    return result, nil
}

func FromCodon(codon string) (string, error) {
    switch codon {
    case "AUG":
        return "Methionine", nil
    case "UUU", "UUC":
        return "Phenylalanine", nil
    case "UUA", "UUG":
        return "Leucine", nil
    case "UCU", "UCC", "UCA", "UCG":
        return "Serine", nil
    case "UAU", "UAC":
        return "Tyrosine", nil
    case "UGU", "UGC":
        return "Cysteine", nil
    case "UGG":
        return "Tryptophan", nil
    case "UAA", "UAG", "UGA":
        return "", ErrStop
    default:
        return "", ErrInvalidBase
    }
}
