use std::collections::BTreeMap;

struct ReputationScore {
    departments: BTreeMap<String, i32>,
    total_score: i32,
}

impl ReputationScore {
    fn new() -> ReputationScore {
        ReputationScore {
            departments: BTreeMap::new(),
            total_score: 0,
        }
    }

    fn add_department(&mut self, department: &str, score: i32) {
        self.departments.insert(department.to_string(), score);
        self.total_score = self.total_score.checked_add(score).unwrap_or(i32::MAX);
    }

    fn update_department(&mut self, department: &str, score: i32) {
        if let Some(existing_score) = self.departments.get_mut(department) {
            self.total_score = self.total_score.checked_sub(*existing_score).unwrap_or(i32::MIN);
            *existing_score = score;
            self.total_score = self.total_score.checked_add(score).unwrap_or(i32::MAX);
        } else {
            self.add_department(department, score);
        }
    }

    fn get_department_score(&self, department: &str) -> Option<i32> {
        self.departments.get(department).copied()
    }

    fn get_all_departments(&self) -> Vec<(&String, &i32)> {
        self.departments.iter().collect()
    }

    fn add_score(&mut self, department: &str, amount: i32) {
        if let Some(score) = self.departments.get_mut(department) {
            *score = score.checked_add(amount).unwrap_or(i32::MAX);
            self.total_score = self.total_score.checked_add(amount).unwrap_or(i32::MAX);
        }
    }

    fn subtract_score(&mut self, department: &str, amount: i32) -> bool {
        if let Some(score) = self.departments.get_mut(department) {
            if *score >= amount {
                *score = score.checked_sub(amount).unwrap_or(0);
                self.total_score = self.total_score.checked_sub(amount).unwrap_or(i32::MIN);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn use_score(&mut self, amount: i32) -> bool {
        if self.total_score >= amount {
            self.total_score = self.total_score.checked_sub(amount).unwrap_or(i32::MIN);
            true
        } else {
            false
        }
    }

    fn get_total_score(&self) -> i32 {
        self.total_score
    }
}

struct Account {
    id: String,
    reputation_score: ReputationScore,
}

impl Account {
    fn new(id: &str) -> Account {
        Account {
            id: id.to_string(),
            reputation_score: ReputationScore::new(),
        }
    }

    fn add_department_score(&mut self, department: &str, score: i32) {
        self.reputation_score.add_department(department, score);
    }

    fn update_department_score(&mut self, department: &str, score: i32) {
        self.reputation_score.update_department(department, score);
    }

    fn get_department_score(&self, department: &str) -> Option<i32> {
        self.reputation_score.get_department_score(department)
    }

    fn get_all_department_scores(&self) -> Vec<(&String, &i32)> {
        self.reputation_score.get_all_departments()
    }

    fn add_score(&mut self, department: &str, amount: i32) {
        self.reputation_score.add_score(department, amount);
    }

    fn subtract_score(&mut self, department: &str, amount: i32) -> bool {
        self.reputation_score.subtract_score(department, amount)
    }

    fn use_reputation_score(&mut self, amount: i32) -> bool {
        self.reputation_score.use_score(amount)
    }

    fn get_total_reputation_score(&self) -> i32 {
        self.reputation_score.get_total_score()
    }
}

fn main() {
    let mut account = Account::new("account1");

    account.add_department_score("sales", 10);
    account.add_department_score("marketing", 20);
    account.add_department_score("customer_service", 30);

    println!("Account 1 scores:");
    for (department, score) in account.get_all_department_scores() {
        println!("{}: {}", department, score);
    }

    println!("Total reputation score: {}", account.get_total_reputation_score());

    account.update_department_score("sales", 15);

    println!("Updated sales score: {:?}", account.get_department_score("sales"));

    account.add_score("sales", 5);

    println!("Added 5 score to sales: {:?}", account.get_department_score("sales"));

    let subtracted = account.subtract_score("sales", 10);
    println!("Subtracted 10 score from sales: {}", subtracted);
    println!("Remaining sales score: {:?}", account.get_department_score("sales"));

    let used = account.use_reputation_score(20);
    println!("Used 20 reputation score: {}", used);
    println!("Remaining total reputation score: {}", account.get_total_reputation_score());
}
