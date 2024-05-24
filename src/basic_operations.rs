use std::io::stdin;
use na::{Dyn, OMatrix, U1};
type ProbabilityMatrixNxN = OMatrix<f64, Dyn, Dyn>;
type ProbabilityMatrix1xN = OMatrix<f64, U1, Dyn>;


pub fn ask_terminal() -> String {
    let mut output = String::new();
    //gets saved in input_str
    stdin().read_line(&mut output)
        .ok()
        .expect("Failed to read line.");
    return output;
}

pub fn number_to_subscript(j: u16) -> String {
    match j.to_string().chars().next() {
        Some(c) => {
            let subscript_char = match c {
                '0' => '₀',
                '1' => '₁',
                '2' => '₂',
                '3' => '₃',
                '4' => '₄',
                '5' => '₅',
                '6' => '₆',
                '7' => '₇',
                '8' => '₈',
                '9' => '₉',
                _ => c, // If the character is not a digit, keep it unchanged
            };
            let mut result = subscript_char.to_string();
            result.push_str(&j.to_string()[c.len_utf8()..]); // Append the rest of the string after the subscript
            result
        }
        None => j.to_string(), // Return the original string if it's empty
    }
}

pub fn str_to_u16(s: String) -> u16{
    return s.trim().parse::<u16>().unwrap()
}
pub fn str_to_f64(s: String) -> f64{
    return s.trim().parse::<f64>().unwrap()

}

pub fn ask_alphabet_length() -> u16{
    println!("Please enter the length of your alphabet:");
    return str_to_u16(ask_terminal());
}
pub fn ask_pretty_for_f64(index: u16) -> f64{
    //just there so the input has these subscript indices
    println!("p(x{}) = ", number_to_subscript(index));
    return str_to_f64(ask_terminal());
}

pub fn ask_pretty_for_vector_of_probabilities(alph_len: u16) -> Vec<f64>{

    let mut probability_vector: Vec<f64> = Vec::new();

    for i in 1..alph_len+1 {
        let current_prob_float: f64 = ask_pretty_for_f64(i);
        //add to probability_vector
        probability_vector.push(current_prob_float);
    }
    return probability_vector;
}

pub fn ask_pretty_for_vector_of_probabilities_with_custom_message(alph_len: u16, custom_message: &str) -> Vec<f64> {
    println!("\n{}", custom_message);
    return ask_pretty_for_vector_of_probabilities(alph_len);
}
pub fn theoretical_maximum_entropy(alph_len: u16) -> f64{
    return (alph_len as f64).log2();
}

pub fn two_to_the_power_of_x_bit(entropy: f64) -> f32{
    return f32::trunc((2_f64.powf(entropy) * 100.0) as f32) / 100.0;
}

pub fn round_to_3dec(x: f64) -> f64 {
    let y = 10i32.pow(3) as f64;
    return (x * y).round() / y;
}

pub fn ask_terminal_with_message(message : &str) -> String {
    println!("{}", message);

    return ask_terminal();
}

/********
    function that takes verbundwarscheinlichkeitsmatrix and returns matrix of
    1st input: "x" or "y" to specify if return matrix is
 */
pub fn verbund_matrix_to_probality_of(variable: &str, alpha_len: u16, matrix1: &ProbabilityMatrixNxN) -> ProbabilityMatrixNxN {
        let x_prob_vec: Vec < f64 > = vec![];

        match variable.to_lowercase().as_str() {
            "x" => {
                let x_prob_vec: Vec < f64 > = matrix1.row_iter().map(| row | row.sum()).collect();
            },
            "y" => {
                let x_prob_vec: Vec < f64 > = matrix1.column_iter().map(| row | row.sum()).collect();
            },
            _ => panic!("Specify x or y for pub fn verbund_matrix_to_probability_of(variable: String, alpha_len: u16, matrix1: &ProbabilityMatrixNxN) -> ProbabilityMatrixNxN ")
        }

        return ProbabilityMatrixNxN::from_row_slice(1, alpha_len as usize, &x_prob_vec);
}

//pub fn probality_of_xi_and_verbundmatrix_to_()