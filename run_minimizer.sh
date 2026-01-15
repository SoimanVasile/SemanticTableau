#!/bin/bash

# Culori
YELLOW='\033[1;33m'
CYAN='\033[1;36m'
GREEN='\033[1;32m'
NC='\033[0m'

# Funcția care rulează un test case
run_demo() {
    local title=$1
    local vars=$2
    local minterms=$3
    
    clear
    echo -e "${YELLOW}========================================${NC}"
    echo -e "${CYAN}DEMO MINIMIZARE:${NC} ${title}"
    echo -e "${YELLOW}========================================${NC}"
    echo -e "Variabile: ${GREEN}$vars${NC}"
    echo -e "Mintermi care vor fi introduși:"
    echo -e "$minterms"
    echo -e "${YELLOW}----------------------------------------${NC}"
    echo -e "Rulare program..."
    echo ""

    echo -e "2\n$vars\n$minterms\ngata\n\n0" | cargo run -q

    echo -e "\n${YELLOW}----------------------------------------${NC}"
    echo -e "Apasă [ENTER] pentru următorul test..."
    read
}

# --- TESTE ---

run_demo "3 Variabile (!x)" \
    "x y z" \
    "0 0 0\n0 0 1\n0 1 0\n0 1 1"

run_demo "4 Variabile (Pătrat Central)" \
    "a b c d" \
    "0 1 0 1\n0 1 1 1\n1 1 0 1\n1 1 1 1"

run_demo "5 Variabile (Complex)" \
    "a b c d e" \
    "1 0 0 0 1\n1 0 0 1 1\n1 0 1 0 1\n1 0 1 1 1"

clear
echo -e "${GREEN}Gata!${NC}"
