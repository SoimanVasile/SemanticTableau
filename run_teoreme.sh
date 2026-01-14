#!/bin/bash

YELLOW='\033[1;33m'
CYAN='\033[1;36m'
GREEN='\033[1;32m'
RED='\033[1;31m'
NC='\033[0m'

declare -a descriptions=(
    "Legea Identității"
    "Legea Terțului Exclus"
    "Legea lui De Morgan"
    "Modus Ponens"
    "Modus Tollens"
    "Silogism Ipotetic"
    "Legea lui Peirce"
    "Eroarea Reciprocei (Non-tautologie)"
)

declare -a formulas=(
    "prove P -> P"
    "prove P | !P"
    "prove !(P & Q) -> (!P | !Q)"
    "prove ((P -> Q) & P) -> Q"
    "prove ((P -> Q) & !Q) -> !P"
    "prove ((P -> Q) & (Q -> R)) -> (P -> R)"
    "prove ((P -> Q) -> P) -> P"
    "prove (P -> Q) -> (Q -> P)"
)

show_slide() {
    idx=$1
    total=${#formulas[@]}
    
    clear 
    
    desc="${descriptions[$idx]}"
    form="${formulas[$idx]}"
    human_idx=$((idx + 1))

    echo -e "${YELLOW}========================================${NC}"
    echo -e "${CYAN}DEMO [${human_idx}/${total}]:${NC} ${desc}"
    echo -e "${YELLOW}========================================${NC}"
    echo -e "Formula: ${GREEN}$form${NC}\n"
    
    echo "$form" | cargo run -q

    echo ""
    echo -e "${YELLOW}----------------------------------------${NC}"
    echo -e " [ENTER/n] Next  |  [b] Back  |  [q] Quit "
}

current=0
total_slides=${#formulas[@]}

while true; do
    if [ $current -lt 0 ]; then current=0; fi
    if [ $current -ge $total_slides ]; then 
        break
    fi

    show_slide $current

    read -rsn1 input

    case "$input" in
        "b"|"B") 
            current=$((current - 1)) 
            ;;
        "q"|"Q")
            echo -e "\nIeșire din Demo..."
            exit 0
            ;;
        *) 
            current=$((current + 1)) 
            ;;
    esac
done

clear
echo -e "${YELLOW}========================================${NC}"
echo -e "${CYAN}DEMO FINALIZAT.${NC}"
echo -e "Intrare în Modul Manual (Scrie 'exit' pentru a ieși)..."
echo -e "${YELLOW}========================================${NC}"

paplay ~/Downloads/heibaieti.mp3

cargo run -q

