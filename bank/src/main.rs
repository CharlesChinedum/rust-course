#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            holder,
            balance: 0,
        }
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }
}

// helper

fn print_bank(bank: &Bank) {
    println!("{:#?}", bank);
}

fn print_account(account: &Account) {
    println!("{:#?}", account)
}

fn change_account(account: &mut Account) {
    account.balance = 10;
}

fn main() {
    let num = 5;

    let other_num = num;

    println!("{} {}", num, other_num);
}
