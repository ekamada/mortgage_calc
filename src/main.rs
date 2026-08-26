

fn main() {

    let principle : u32 = 400000;
    let int_rate  : f64 = 4.04;
    let compounding_periods = 2;
    let num_annl_payments = 12;
    let amortization_period = 25;

    let mortgage_data = Mortgage::new(principle, int_rate, amortization_period);


    let int_payment = calc_interest(int_rate, compounding_periods, num_annl_payments);
    let effective_rate = calc_eff_annual_rate(int_rate, compounding_periods);
    // gen_amortization_payments(int_rate, compounding_periods, num_annl_payments, principle, 1);
    //
    let montly_payment = calc_monthly_payment(principle, int_rate, amortization_period, compounding_periods);
    
    println!("Hello, world!");
    println!("");
    println!("{effective_rate}");
    println!("{}",principle as f64 * int_payment);
    println!("{montly_payment}");
    
    println!("");
    println!("{}", mortgage_data.calc_effective_rate());
    println!("{}", mortgage_data.calc_interest_payment());
    println!("{}", mortgage_data.calc_monthly_payment());

}

struct Mortgage {
    principle : f64,
    int_rate  : f64,
    compunding_periods  : f64,
    num_annual_payments : f64,
    amortization_period : f64
}

impl Mortgage {
    pub fn new (principle : u32, rate : f64, amortization_per : u32) -> Mortgage {
        let compounding_periods = 2;
        let num_annual_payments = 12;

        Mortgage { principle           : principle as f64,  
                   int_rate            : rate/100.0, 
                   compunding_periods  : compounding_periods as f64, 
                   num_annual_payments : num_annual_payments as f64, 
                   amortization_period : amortization_per as f64}   
    }

    pub fn calc_effective_rate(&self) -> f64 {
        let result : f64 = 1.0 + (self.int_rate)/self.compunding_periods;

        result.powf(self.compunding_periods) - 1.0
    }

    pub fn calc_interest_payment(&self) -> f64 {

        let result : f64 = 1.0 + (self.int_rate/self.compunding_periods);

        self.principle * (result.powf(self.compunding_periods/self.num_annual_payments) - 1.0)
    }

    pub fn calc_monthly_payment(&self) -> f64 {
        let eff_rate : f64 = self.calc_effective_rate();
        let periodic_rate : f64 = eff_rate/12.0;
        let total_payments : f64 = (self.num_annual_payments * self.amortization_period) as f64;
        let core_rate : f64 = 1.0 + periodic_rate;

        let numerator = periodic_rate * core_rate.powf(total_payments);
        let denominator = core_rate.powf(total_payments) - 1.0;

        self.principle * numerator / denominator
    }
}


// Calculates the effective annual interest rate
fn calc_eff_annual_rate (interest_rate: f64, annual_compounding_periods : u16) -> f64 {
    let f_compounds = annual_compounding_periods as f64;
    let result : f64 = 1.0 + (interest_rate/100.0)/(f_compounds);
    
    return result.powf(f_compounds)-1.0;

}

// calculate interest payments
fn calc_interest (interest_rate: f64, annual_compounding_periods : u16, annual_payments : u16) -> f64 {

    let f_compounds = annual_compounding_periods as f64;
    let f_payments = annual_payments as f64;
    let result : f64 = 1.0 + (interest_rate/100.0)/(f_compounds);

    return result.powf(f_compounds/f_payments)-1.0;
}

fn calc_monthly_payment(principle : u32, rate : f64, amortization_per : u32, annual_compounding_periods : u16) -> f64 {

    let annual_payments      = 12;
    let f_principle = principle as f64;
    let eff_rate = calc_eff_annual_rate(rate, annual_compounding_periods);
    let periodic_rate : f64 = eff_rate/12.0;
    let total_payments : f64 = (annual_payments * amortization_per) as f64;
    let core_rate : f64      = 1.0 + periodic_rate;

    let numerator   = periodic_rate * core_rate.powf(total_payments);
    let denominator = core_rate.powf(total_payments) - 1.0;

    return f_principle * numerator / denominator;
    

}

// fn gen_amortization_payments (rate: f64, num_periods : u16, annual_payments : u16, principle: u32, term : u16) {
//
//     let mut amount_due = principle;
//     let total_payments = annual_payments*term;
//
//     for i in 1..total_payments {
//
//     }
//     println!("Hello, world!");
// }
//

