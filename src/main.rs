use std::fmt::format;

#[derive(Debug)]

struct Account{
    id: u32,
    balance: i32,
    holder: String,

}
impl Account{
    fn new(id:u32, holder:String)-> Self{
        Account { id,  holder, balance: 0 }

    }

    fn deposit(&mut self, amount: i32)->i32{
        self.balance += amount;
        self.balance

    }

    fn withdraw(&mut self, amount: i32) -> i32{
        self.balance -= amount;
        self.balance
    }

    fn summary(&self)-> String{
        format!("{} has a a balance of {}", self.holder, self.balance)
    }
}

#[derive(Debug)]

struct Bank{
    accounts: Vec<Account>,

}

impl Bank {
    fn new()->Self{
        Bank{
            accounts: vec![],
        }
    }


    fn add_account(&mut self,  account:Account){
        self.accounts.push(account)

    }

    fn total_balance(&self)->i32{
        self.accounts.iter().map(|account|account.balance).sum()

    }

    fn summary(&self)-> Vec<String>{
        self.accounts.iter().map(|account|account.summary()).collect::<Vec<String>>()


    }
    
}

// fn print_sccount(account: &Account)
// {
//     println!("{:?}", account.balance);
   
// }



fn main() {
//    let account = Account::new(1, "John".to_string());
//    let account_ref = &account;

//    print_sccount(account_ref);
// //    account = print_sccount(account);

//    println!("{:?}", account);

        let mut bank = Bank::new();
        let mut account= Account::new(1, String::from("me"));

        account.deposit(500);
        account.withdraw(200);

        // println!("{}", account.summary());


        bank.add_account(account);
        println!("{:#?}", bank.summary());
        println!("{}", bank.total_balance());

}


