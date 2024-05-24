use na::{U2, U3, Dyn, ArrayStorage, VecStorage, Matrix, SMatrix, OMatrix, U1};
use crate::{basic_operations, calculation_entropy, main};
use crate::basic_operations::{ask_terminal, ask_terminal_with_message};

pub fn of_static_source(){
    let alphabet_length_int = basic_operations::ask_alphabet_length();
    let mut entropy:f64 = 0.0;
    let probability_vector: Vec<f64> = basic_operations::ask_pretty_for_vector_of_probabilities(alphabet_length_int);

    //Code for Hm = -E(p(xi)*ld(p(xi))  | E stands for Sigma (The Sum sign)
    for prob in probability_vector{
        entropy = entropy - (prob.log(2.0) * prob);
    }
    println!("Entropy: Hₘ = {} bit/Ereignis", entropy);
    println!("Theoretical Maximum Entropy: {} bit/Ereignis", basic_operations::theoretical_maximum_entropy(alphabet_length_int));
    println!("With your source, you are just as uncertain as if you had {} equally likely probabilities.", basic_operations::two_to_the_power_of_x_bit(entropy));
}
pub fn markow_source(){

    let alphabet_length_int = basic_operations::ask_alphabet_length();
    type ProbabilityMatrixNxN = OMatrix<f64, Dyn, Dyn>;
    type ProbabilityMatrix1xN = OMatrix<f64, U1, Dyn>;

    let probability_vector: Vec<f64> = basic_operations::ask_pretty_for_vector_of_probabilities_with_custom_message(alphabet_length_int, "Alphabet probabilities p(xᵢ): ");
    let mut probability_matrix_at_t0 = ProbabilityMatrix1xN::from_row_slice(&probability_vector);
    eprintln!("{probability_matrix_at_t0:.2}");

    let transition_vector = basic_operations::ask_pretty_for_vector_of_probabilities_with_custom_message(alphabet_length_int * alphabet_length_int, "Transition probabilities p(xⱼ|xᵢ): ");
    let transition_matrix = ProbabilityMatrixNxN::from_row_slice(alphabet_length_int as usize, alphabet_length_int as usize, &transition_vector);
    eprintln!("{transition_matrix:.2}");

    let mut probability_not_stationary_flag: bool = true;
    let mut j = 0; // Assuming `j` is declared and initialized elsewhere

    while probability_not_stationary_flag {
        j += 1;
        let probability_matrix_at_t1 = probability_matrix_at_t0.clone() * transition_matrix.clone();

        println!("ITERATION {}:  new p(xᵢ): ", j);
        eprintln!("{probability_matrix_at_t1}:.2");

        let mut flag_changed = false;
        for i in 0..alphabet_length_int {
            if basic_operations::round_to_3dec(probability_matrix_at_t0[(0 as usize, i as usize)]) != basic_operations::round_to_3dec(probability_matrix_at_t1[(0 as usize, i as usize)]) {
                println!("at {} NOT the same" ,i);
                flag_changed = true;
                break;
            }
        }

        if flag_changed {
            probability_not_stationary_flag = true;
        } else {
            probability_not_stationary_flag = false;
            println!("ITERATION {}: Alphabet probabilities now stationary", j);
        }

        probability_matrix_at_t0 = probability_matrix_at_t1.clone();
    }

}

pub fn verbundquelle(){
    let alphabet_length_int : u16 = basic_operations::ask_alphabet_length();
    type ProbabilityMatrixNxN = OMatrix<f64, Dyn, Dyn>;

    let verbundwarscheinlichkeit_matrix: OMatrix<f64, Dyn, Dyn>;
    let bedingte_warscheinlichkeit_matrix_x_affects_y: OMatrix<f64, Dyn, Dyn>;
    let bedingte_warscheinlichkeit_matrix_y_affects_x: OMatrix<f64, Dyn, Dyn>;

    match ask_terminal_with_message(r#"What do you want to calculate?
        1)Verbundwarscheinlichkeitsmatrix (p(xᵢ,yⱼ))
        2)bedingte Warscheinlichkeit von x zu y: p(yⱼ|xᵢ))
        3)bedingte Warscheinlichkeit von y zu x: p(xᵢ|yⱼ))
        "#){
        1 => {
            let verbundwarscheinlichkeit_vector = basic_operations::ask_pretty_for_vector_of_probabilities_with_custom_message(alphabet_length_int, "Verbundwarscheinlichkeiten p(xⱼ|xᵢ): ");
            let y_to_x_vector: Vec<f64> = basic_operations::ask_pretty_for_vector_of_probabilities_with_custom_message();
        },
        2 => {
            let verbundwarscheinlichkeit_vector = basic_operations::ask_pretty_for_vector_of_probabilities_with_custom_message(alphabet_length_int, "Verbundwarscheinlichkeiten p(xⱼ|xᵢ): ");
            let y_to_x_vector: Vec<f64> = basic_operations::ask_pretty_for_vector_of_probabilities_with_custom_message("p(xᵢ|yⱼ): ");
        },
        6969 => {
            let verbundwarscheinlichkeit_vector: Vec<f64> = basic_operations::ask_pretty_for_vector_of_probabilities_with_custom_message(alphabet_length_int, "Verbundwarscheinlichkeiten p(xⱼ|xᵢ): ");
            //let mut transitioned_matrix = ProbabilityMatrixNxN:: },
        _ => {println!("Please enter a number in the specified range!\n\n");
            main();}
    }


    let verbundwarscheinlichkeit_vector: Vec<f64> = basic_operations::ask_pretty_for_vector_of_probabilities_with_custom_message(alphabet_length_int, "Verbundwarscheinlichkeiten p(xⱼ|xᵢ): ");

    let mut verbundwarscheinlichkeit_matrix = ProbabilityMatrixNxN::from_row_slice(&verbundwarscheinlichkeit_vector);
}