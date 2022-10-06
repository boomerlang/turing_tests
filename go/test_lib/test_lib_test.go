package test_lib

import (
	"testing"
)

func TestCalcPoints1(t *testing.T) {
	input := []string{"5","2","C","D","+"}

	sum := CalcPoints(input)

	if sum != 30 {
		t.Fatalf(`Expected = %v, Got = %v`, 30, sum)
	}
}

func TestCalcPoints2(t *testing.T) {
	input := []string{"5","-2","4","C","D","9","+","+"}

	sum := CalcPoints(input)
	
	// Their solution is 27 , which I think is wrong!
	// I think is 15
	if sum != 27 {
		t.Fatalf(`Expected = %v, Got = %v`, 27, sum)
	}
}

func TestCalcPoints3(t *testing.T) {
	input := []string{"1"}

	sum := CalcPoints(input)
	
	if sum != 1 {
		t.Fatalf(`Expected = %v, Got = %v`, 1, sum)
	}
}

func TestValidParant0(t *testing.T) {
	input1 := "()"
	rez := ValidParant(input1)
	
	if rez == false {
		t.Fatalf(`Expected = %v, Got = %v`, true, rez)
	}
}

func TestValidParant01(t *testing.T) {
	input1 := "(]"
	rez := ValidParant(input1)
	
	if rez == true {
		t.Fatalf(`Expected = %v, Got = %v`, false, rez)
	}
}

func TestValidParant1(t *testing.T) {
	input1 := "([)]"
	rez := ValidParant(input1)
	
	if rez == true {
		t.Fatalf(`Expected = %v, Got = %v`, false, rez)
	}
}

func TestValidParant2(t *testing.T) {
	input1 := "{[]}"
	rez := ValidParant(input1)
	
	if rez == false {
		t.Fatalf(`Expected = %v, Got = %v`, true, rez)
	}
}

func TestValidParant3(t *testing.T) {
	input1 := "()[]{}"
	rez := ValidParant(input1)
	
	if rez == false {
		t.Fatalf(`Expected = %v, Got = %v`, true, rez)
	}
}

func TestValidParant4(t *testing.T) {
	input1 := "()[({})]{[({})]}"
	rez := ValidParant(input1)
	
	if rez == false {
		t.Fatalf(`Expected = %v, Got = %v`, true, rez)
	}
}
