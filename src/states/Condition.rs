mod Condition{
    fn test() -> bool{}


    mod AndCondition extends Condition{
         conditionA: Condition
         conditionB: Condition
        fn test() -> bool{
            return conditionA.test() and conditionB.test()  
        }
  
    }

    mod NotCondition extends Condition{
    condition: Condition
    fn test() -> bool:
    return not condition.test()
    }
    mod OrCondition extends Condition{
         conditionA: Condition
         conditionB: Condition
        fn test() -> bool{
            return conditionA.test() or conditionB.test()
        }

    }

}
