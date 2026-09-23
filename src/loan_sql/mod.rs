
pub mod mortgage;

use std::{fmt,io::{self, Write}};

use rusqlite::{Connection,params};
use pad::PadStr;

use mortgage::Mortgage;

pub struct LoanData {
    conn : Connection
}

impl LoanData {
    pub fn new() -> LoanData {
        let conn = Connection::open("loans.db").unwrap();
        let cmd_str = format!(" 
            create table if not exists LoanData (
                ID           INT PRIMARY KEY NOT NULL,
                START_DATE   Text            NOT NULL,
                PRINCIPLE    Text            NOT NULL,
                OUTSTANDING  Text            NOT NULL,
                RATE         Text            NOT NULL,
                PAYMENT_TYPE Text            NOT NULL,
                AMORTIZATION Text            NOT NULL
                )
            ");
        let _cmd = conn.execute(&cmd_str, ());
        LoanData{conn : conn}
    }
    
    pub fn list_tasks(&self) {
        let cmd_str = format!("SELECT * from LoanData");
        let mut stmt = self.conn.prepare(&cmd_str).unwrap();
        let mut rows = stmt.query([]).unwrap();

        while let Some(row) = rows.next().unwrap() {
            let data = Mortgage::new(
                row.get(1).unwrap(),  // Start Date
                row.get(2).unwrap(),  // Principle
                row.get(4).unwrap(),  // Interest Rate
                row.get(6).unwrap()); // Amortization Period
            println!("{}", data);
        }

    }
}

