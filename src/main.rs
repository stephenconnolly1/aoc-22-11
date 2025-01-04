

fn main() {

}

struct Monkey {
    items: Vec<i64>,
    op: fn(i64) -> i64, 
    modulo: i64,
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
    fn inspect (&mut self, modulo_product: i64) -> Vec<InspectResult> {
        let mut results: Vec<InspectResult> = Vec::new();
        for i in self.items.iter() {
            let mut new = (self.op)(*i);
            new = new % modulo_product; 
            let target_monkey = if new % self.modulo == 0 {&self.to_if_true} else {&self.to_if_false};
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

fn process_monkeys(mut monkeys: Vec<Monkey>) {
    let mut modulo_product:i64 = 1;
    for i in monkeys.iter() { modulo_product = modulo_product * i.modulo; }
    println! ("Modulo: {}", modulo_product);

    let mut monkey=0;
    for round in 1..10001 {
        for i in 0..monkeys.len() {
            //println!("Monkey{}", i);
            let results = monkeys[i].inspect(modulo_product);
            for r in results.iter() {
                //println!("     Result: {:?}", r);
                monkeys[r.target_monkey].items.push(r.worry_level);
            }
            monkey += 1;

        }    
        match round {
            1|20|1000|2000|3000|4000|5000|6000|7000|8000|9000|10000 => {
                println!("== After round {} ==", round);
                for m in monkeys.iter() {
                    println!("Monkey inspected items {} times. Values {:?} ", m.num_inspections, m.items);
                }
            }
            _ => (),
        }
    }
    let mut business: Vec<i64> =  monkeys.iter().map(|m| m.num_inspections).collect::<Vec<_>>();
    business.sort_by(|a, b| b.cmp(a));
    println!("business: {:?}", business[0] * business [1]);

}
#[cfg(test)]
mod test1 {
    use super::*;

    #[test]
    fn t1() {
        let mut monkeys: Vec<Monkey> = Vec::new();
        monkeys.push(Monkey {
            items: vec!(79, 98),
            op: |a| a * 19,
            modulo: 23,
            to_if_true:2,
            to_if_false:3,
            num_inspections: 0,
        });

        monkeys.push(Monkey {
            items: vec!(54, 65, 75, 74),
            op: |a| a + 6 ,
            modulo : 19,
            to_if_true:2,
            to_if_false:0,
            num_inspections: 0,
        });

        monkeys.push(Monkey {
            items: vec!(79, 60, 97),
            op: |a| a * a,
            modulo: 13,
            to_if_true:1,
            to_if_false:3,
            num_inspections: 0,
        });

        monkeys.push(Monkey {
            items: vec!(74),
            op: |a| a + 3,
            modulo: 17,
            to_if_true:0,
            to_if_false:1,
            num_inspections: 0,
        });

        process_monkeys(monkeys);

        assert_eq!(true,true) ;
    }


    #[test]
     fn t2() {
        let mut monkeys: Vec<Monkey> = Vec::new();
        monkeys.push(Monkey {
            items: vec!(75, 75, 98, 97, 79, 97, 64),
            op: |a| a * 13,
            modulo: 19,
            to_if_true:2,
            to_if_false:7,
            num_inspections: 0,
        });

        monkeys.push(Monkey {
            items: vec!(50, 99, 80, 84, 65, 95),
            op: |a| a + 2 ,
            modulo: 3,
            to_if_true: 4,
            to_if_false: 5,
            num_inspections: 0,
        });

        monkeys.push(Monkey {
            items: vec!(96, 74, 68, 96, 56, 71, 75, 53),
            op: |a| a + 1,
            modulo: 11,
            to_if_true: 7,
            to_if_false: 3,
            num_inspections: 0,
        });

        monkeys.push(Monkey {
            items: vec!(83, 96, 86, 58, 92),
            op: |a| a + 8,
            modulo: 17,
            to_if_true: 6,
            to_if_false: 1,
            num_inspections: 0,
        });

        monkeys.push(Monkey {
            items: vec!(99),
            op: |a| a * a,
            modulo: 5,
            to_if_true: 0,
            to_if_false: 5,
            num_inspections: 0,
        });
        monkeys.push(Monkey {
            items: vec!(60, 54, 83),
            op: |a| a + 4,
            modulo: 2,
            to_if_true: 2,
            to_if_false: 0,
            num_inspections: 0,
        });
        monkeys.push(Monkey {
            items: vec!(77, 67),
            op: |a| a * 17,
            modulo: 13,
            to_if_true: 4,
            to_if_false: 1,
            num_inspections: 0,
        });
        monkeys.push(Monkey {
            items: vec!(95, 65, 58, 76),
            op: |a| a + 5,
            modulo: 7,
            to_if_true: 3,
            to_if_false: 6,
            num_inspections: 0,
        });
        process_monkeys(monkeys);
        assert_eq!(true,true) ;
    }

}   

