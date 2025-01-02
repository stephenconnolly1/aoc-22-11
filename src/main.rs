

fn main() {

}

struct Monkey {
    items: Vec<i64>,
    op: fn(i64) -> i64, 
    test: fn(i64) -> bool,
    to_if_true: usize,
    to_if_false: usize,
    num_inspections: i64,
}
#[derive(Debug)]
struct InspectResult {
    worry_level: i64,
    target_monkey: usize,
}

impl Monkey {
    fn inspect (&mut self) -> Vec<InspectResult> {
        let mut results: Vec<InspectResult> = Vec::new();
        for i in self.items.iter() {
            println!("Inspecting: {}", i);
            let mut new = (self.op)(*i);
            println!("new value: {}", new);
            new = new / 3;
            println!("new value / 3: {}", new);
            let test_result = (self.test)(new);
            println!("test_result: {}", test_result);
            let target_monkey = if test_result {&self.to_if_true} else {&self.to_if_false};
            println!("throw {} to monkey {}", new, target_monkey) ;
            println!("----");
            results.push(InspectResult{
                worry_level: new,
                target_monkey: *target_monkey,
            });
            self.num_inspections += 1;
    
        }
        self.items.clear();
        return results;
    }
}

#[cfg(test)]
mod test1 {
    use super::*;
    /* 
    #[test]
    fn t1() {
        let mut monkeys: Vec<Monkey> = Vec::new();
        monkeys.push(Monkey {
            items: vec!(79,98),
            op: |a| a * 19,
            test: |a| a % 23 == 0,
            to_if_true:2,
            to_if_false:3,
            num_inspections: 0,
        });

        monkeys.push(Monkey {
            items: vec!(54, 65, 75, 74),
            op: |a| a + 6 ,
            test: |a| a % 19 == 0,
            to_if_true:2,
            to_if_false:0,
            num_inspections: 0,
        });

        monkeys.push(Monkey {
            items: vec!(79,60, 97),
            op: |a| a * a,
            test: |a| a % 13 == 0,
            to_if_true:1,
            to_if_false:3,
            num_inspections: 0,
        });

        monkeys.push(Monkey {
            items: vec!(74),
            op: |a| a + 3,
            test: |a| a % 17 == 0,
            to_if_true:0,
            to_if_false:1,
            num_inspections: 0,
        });

        let mut monkey=0;
        for round in 0..20 {
            println!("Round {}", round);
            for i in 0..monkeys.len() {
                println!("Monkey{}", i);
                let results = monkeys[i].inspect();
                for r in results.iter() {
                    println!("     Result: {:?}", r);
                    monkeys[r.target_monkey].items.push(r.worry_level);
                }
                monkey += 1;
            }    

        }
        for m in monkeys.iter() {
            println!("Items: {:?}, Inspections: {}", m.items, m.num_inspections);
        }
        assert_eq!(true,true) ;
    }
*/
    ///////
    #[test]
    fn t2() {
        let mut monkeys: Vec<Monkey> = Vec::new();
        monkeys.push(Monkey {
            items: vec!(75, 75, 98, 97, 79, 97, 64),
            op: |a| a * 13,
            test: |a| a % 19 == 0,
            to_if_true:2,
            to_if_false:7,
            num_inspections: 0,
        });

        monkeys.push(Monkey {
            items: vec!(50, 99, 80, 84, 65, 95),
            op: |a| a + 2 ,
            test: |a| a % 3 == 0,
            to_if_true: 4,
            to_if_false: 5,
            num_inspections: 0,
        });

        monkeys.push(Monkey {
            items: vec!(96, 74, 68, 96, 56, 71, 75, 53),
            op: |a| a + 1,
            test: |a| a % 11 == 0,
            to_if_true: 7,
            to_if_false: 3,
            num_inspections: 0,
        });

        monkeys.push(Monkey {
            items: vec!(83, 96, 86, 58, 92),
            op: |a| a + 8,
            test: |a| a % 17 == 0,
            to_if_true: 6,
            to_if_false: 1,
            num_inspections: 0,
        });


        monkeys.push(Monkey {
            items: vec!(99),
            op: |a| a * a,
            test: |a| a % 5 == 0,
            to_if_true: 0,
            to_if_false: 5,
            num_inspections: 0,
        });
        monkeys.push(Monkey {
            items: vec!(60, 54, 83),
            op: |a| a + 4,
            test: |a| a % 2 == 0,
            to_if_true: 2,
            to_if_false: 0,
            num_inspections: 0,
        });
        monkeys.push(Monkey {
            items: vec!(77, 67),
            op: |a| a * 17,
            test: |a| a % 13 == 0,
            to_if_true: 4,
            to_if_false: 1,
            num_inspections: 0,
        });
        monkeys.push(Monkey {
            items: vec!(95, 65, 58, 76),
            op: |a| a + 5,
            test: |a| a % 7 == 0,
            to_if_true: 3,
            to_if_false: 6,
            num_inspections: 0,
        });

        let mut monkey=0;
        for round in 0..20 {
            println!("Round {}", round);
            for i in 0..monkeys.len() {
                println!("Monkey{}", i);
                let results = monkeys[i].inspect();
                for r in results.iter() {
                    println!("     Result: {:?}", r);
                    monkeys[r.target_monkey].items.push(r.worry_level);
                }
                monkey += 1;
            }    

        }
        for m in monkeys.iter() {
            println!("Items: {:?}, Inspections: {}", m.items, m.num_inspections);
        }
        assert_eq!(true,true) ;
    }

}   

