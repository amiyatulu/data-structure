# Transcription

## Instructions

Your task is determine the RNA complement of a given DNA sequence.

Both DNA and RNA strands are a sequence of nucleotides.

The four nucleotides found in DNA are adenine (A), cytosine (C), guanine (G) and thymine (T).

The four nucleotides found in RNA are adenine (A), cytosine (C), guanine (G) and uracil (U).

Given a DNA strand, its transcribed RNA strand is formed by replacing each nucleotide with its complement:

    G -> C
    C -> G
    T -> A
    A -> U
    

### Notes on Rust implementation

By using private fields in structs with public new functions returning Option or Result (as here with DNA::new & RNA::new), we can guarantee that the internal representation of DNA is correct. Because every valid DNA string has a valid RNA string, we don't need to return a Result/Option from into_rna.

This explains the type signatures you will see in the tests.

The return types of both DNA::new() and RNA::new() are Result<Self, usize>, where the error type usize represents the index of the first invalid character (char index, not utf8).

