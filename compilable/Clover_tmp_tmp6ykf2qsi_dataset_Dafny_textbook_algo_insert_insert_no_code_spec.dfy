// specification: This insert method inserts elements from array nl into array line at the position at, shifting subsequent elements to the right. The length of line and nl before insertion are represented by l and p respectively. It mandates that 0 <= l + p <= line.Length, 0 <= p <= nl.Length, and 0 <= at <= l. The method modifies line such that for all i in 0 <= i < p, line[at + i] equals nl[i], and all other elements of line maintain their original values in the adjusted positions after the insertion. 
method insert(line:array<char>, l:int, nl:array<char>, p:int, at:int)
//TOFILL
