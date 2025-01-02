#[cfg(test)]
mod test1 {
    use super::*;
    #[test]
    fn t1() {
        let mut monkeys<Vec: Monkey> = Vec::new();
        monkeys,push(Monkey {
            items: vec!(79,98),
            op: |a| a * 19,
            test: |a| a % 23 == 0,
            to_if_true:2,
            to_if_false:3,
        });

        let m1 = Monkey {
            items: vec!(54, 65, 75, 74),
            op: |a| a + 6 ,
            test: |a| a % 19 == 0,
            to_if_true:2,
            to_if_false:0,
        };

        let m2 = Monkey {
            items: vec!(79,60, 97),
            op: |a| a * a,
            test: |a| a % 13 == 0,
            to_if_true:1,
            to_if_false:3,
        };

        let m3 = Monkey {
            items: vec!(74),
            op: |a| a + 3,
            test: |a| a % 17 == 0,
            to_if_true:0,
            to_if_false:1,
        };

        m0.inspect();
        assert_eq!(true,true) ;
    }
}   