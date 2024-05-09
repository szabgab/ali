pub mod utils;
pub mod matrix;

use utils::{input, clear};

use matrix::Matrix;

use crate::utils::pause;

fn menu_option() -> u8 {
    clear();
    println!("###################################");
    println!("## THE ALI PROJECT - @jakkunight ##");
    println!("###################################");
    println!();
    println!("######## MATRIX CALCULATOR ########");
    println!("Select an option: ");
    println!("(1) Matrix addition");
    println!("(2) Matrix substraction");
    println!("(3) Matrix product");
    println!("(4) Matrix determinant");
    println!("(0) Exit");
    input::<u8>(Some("Insert a valid option:".to_string()))
}

fn add_matrixes() {
    clear();
    println!("####### MATRIX ADDITION ########");
    let rows = input::<usize>(Some("Input number of rows:".to_string()));
    let columns = input::<usize>(Some("Input number of columns:".to_string()));
    println!();
    println!("########### MATRIX A ###########");
    let mut a = Matrix::new(rows, columns);
    a.edit();
    println!();
    println!("########### MATRIX B ###########");
    let mut b = Matrix::new(rows, columns);
    b.edit();
    println!();
    println!("############ RESULT ############");
    (a + b).display();
    println!();
    pause();
}

fn sub_matrixes() {
    clear();
    println!("##### MATRIX SUBSTRACTION ######");
    let rows = input::<usize>(Some("Input number of rows:".to_string()));
    let columns = input::<usize>(Some("Input number of columns:".to_string()));
    println!("########### MATRIX A ###########");
    let mut a = Matrix::new(rows, columns);
    a.edit();
    println!();
    println!("########### MATRIX B ###########");
    let mut b = Matrix::new(rows, columns);
    b.edit();
    println!();
    println!("############ RESULT ############");
    (a - b).display();
    println!();
    pause();
}


fn multiply_two_matrixes() {
    clear();
    println!("######## MATRIX PRODUCT ########");
    println!("########### MATRIX A ###########");
    let a = Matrix::load();
    println!("########### MATRIX B ###########");
    let b = Matrix::load();
    println!("############ RESULT ############");
    if a.clone().columns() != b.clone().rows() {
        println!("El producto NO existe!");
        pause();
        return;
    }
    (a * b).display();
    println!();
    pause();
}

fn find_determinant() {
    clear();
    println!("###### MATRIX DETERMINANT ######");
    println!("Finds the determinant of a matrix");
    println!("by the Laplace's expansion method");
    println!();
    let order = input::<usize>(Some("Insert matrix ordder:".to_string()));
    println!("############ MATRIX ############");
    let mut a = Matrix::new(order, order);
    a.edit();
    println!("############ RESULT ############");
    println!("{}", a.determinant());
    pause();
}

fn main() {
    loop {
        match menu_option() {
            0 => {
                println!("Bye!");
                break;
            },
            1 => add_matrixes(),
            2 => sub_matrixes(),
            3 => multiply_two_matrixes(),
            4 => find_determinant(),
            _ => println!("INVALID OPTION"),
        }
    }
}
