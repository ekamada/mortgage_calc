

fn main() {

    let principle : u32 = 500000;
    let int_rate  : f64 = 5.00;
    let _compounding_periods = 2;
    let _num_annl_payments = 12;
    let amortization_period = 25;

    let mut mortgage_data = Mortgage::new(principle, int_rate, amortization_period);

    println!("Hello, world!");
    println!("");
    println!("{}", mortgage_data.calc_effective_rate());
    println!("{}", mortgage_data.calc_interest_payment());
    println!("{}", mortgage_data.calc_monthly_payment());
    println!("");

    mortgage_data.gen_amortization_calendar();

}

struct Mortgage {
    principle : f64,
    balance   : f64,
    int_rate  : f64,
    compounding_periods  : f64,
    num_annual_payments : f64,
    amortization_period : f64
}

impl Mortgage {
    pub fn new (principle : u32, rate : f64, amortization_per : u32) -> Mortgage {
        let compounding_periods = 2;
        let num_annual_payments = 12;

        Mortgage { principle           : principle as f64,  
                   balance             : principle as f64,  
                   int_rate            : rate/100.0, 
                   compounding_periods : compounding_periods as f64, 
                   num_annual_payments : num_annual_payments as f64, 
                   amortization_period : amortization_per as f64}   
    }

    pub fn calc_effective_rate(&self) -> f64 {
        let result : f64 = 1.0 + (self.int_rate)/self.compounding_periods;
        result.powf(self.compounding_periods) - 1.0
    }

    fn calc_periodic_rate(&self) -> f64 {
        let result : f64 = 1.0 + (self.int_rate/self.compounding_periods);
        result.powf(self.compounding_periods/self.num_annual_payments) - 1.0
    }

    pub fn calc_interest_payment(&self) -> f64 {
        self.balance * self.calc_periodic_rate()
    }

    pub fn calc_monthly_payment(&self) -> f64 {
        
        let periodic_rate  : f64 = self.calc_periodic_rate();
        let total_payments : f64 = (self.num_annual_payments * self.amortization_period) as f64;
        let core_rate      : f64 = 1.0 + periodic_rate;

        let numerator   = periodic_rate * core_rate.powf(total_payments);
        let denominator = core_rate.powf(total_payments) - 1.0;

        self.principle * numerator / denominator
    }

    pub fn gen_amortization_calendar(&mut self) {
        // 1. Calculate interest paid 
        // 2. calculate monthly payment
        // 3. determine amount paid off from principle
        // 4. update principle amount

        // let total_payments = (self.num_annual_payments * self.amortization_period) as u32;
        // let mut outstanding_amt = self.principle;
        let mut interest_amt : f64;
        let mut payment_amt : f64;
        let mut principle_paid : f64;

        // for _ in 1..total_payments {
        for _ in 1..=12 {
            interest_amt = self.calc_interest_payment();
            payment_amt = self.calc_monthly_payment();
            principle_paid = payment_amt - interest_amt;

            self.balance = self.balance - principle_paid;

            println!("{} {} {} {}", interest_amt, payment_amt, principle_paid, self.balance)

        }
    }
}

