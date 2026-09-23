
pub mod loan_sql;
pub use loan_sql::mortgage::Mortgage;

// use formulas::Mortgage;

fn main() {

    let principle : u32 = 500000;
    let int_rate  : f64 = 5.00;
    let _compounding_periods = 2;
    let _num_annl_payments = 12;
    let amortization_period = 25;

    let mut mortgage_data = Mortgage::new(String::from(""), principle, int_rate, amortization_period);

    println!("Hello, world!");
    println!("");
    println!("{}", mortgage_data.calc_effective_rate());
    println!("{}", mortgage_data.calc_interest_payment());
    println!("{}", mortgage_data.calc_monthly_payment());
    println!("");

    mortgage_data.gen_amortization_calendar();

}

