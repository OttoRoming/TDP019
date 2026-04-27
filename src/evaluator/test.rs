use super::{Value, eval};
use pretty_assertions::assert_eq;
use std::{cell::RefCell, rc::Rc};

// https://stackoverflow.com/questions/30856285/assert-eq-with-floating-point-numbers-and-delta
macro_rules! assert_delta {
    ($x:expr, $y:expr, $d:expr) => {
        if !($x - $y < $d || $y - $x < $d) {
            panic!();
        }
    };
}

#[test]
fn int() {
    assert_eq!(Ok(Value::Int(1)), eval("return 1"))
}

#[test]
fn float() {
    assert_eq!(Ok(Value::Float(0.1)), eval("return 0.1"))
}

#[test]
fn eval_true() {
    assert_eq!(Ok(Value::Bool(true)), eval("return true"))
}

#[test]
fn eval_false() {
    assert_eq!(Ok(Value::Bool(false)), eval("return false"))
}

#[test]
fn string() {
    assert_eq!(
        Ok(Value::String("test".to_string())),
        eval("return \"test\"")
    )
}

#[test]
fn empty_string() {
    assert_eq!(Ok(Value::String("".to_string())), eval("return \"\""))
}

#[test]
fn assign_equals_int() {
    assert_eq!(
        Ok(Value::Int(2)),
        eval(
            "
                var i = 1
                i = 2;
                return i
            "
        )
    )
}

#[test]
fn assign_equals_float() {
    assert_eq!(
        Ok(Value::Float(0.2)),
        eval(
            "
                var f = 0.1
                f = 0.2;
                return f
            "
        )
    )
}

#[test]
fn assign_equals_bool() {
    assert_eq!(
        Ok(Value::Bool(true)),
        eval(
            "
                var b = false
                b = true;
                return true
            "
        )
    )
}

#[test]
fn assign_equals_string() {
    assert_eq!(
        Ok(Value::String("world".to_string())),
        eval(
            "
                var s = \"hello\"
                s = \"world\";
                return s
            "
        )
    )
}

#[test]
fn assign_equals_list() {
    assert_eq!(
        Ok(Value::List(vec![Rc::new(RefCell::new(Value::Int(1)))])),
        eval(
            "
                var l: List<Int> = []
                l = [1];
                return l
            "
        )
    )
}

#[test]
fn assign_equals_reference() {
    assert_eq!(
        Ok(Value::Reference(Rc::new(RefCell::new(Value::Int(2))))),
        eval(
            "
                var r: Ref<Int> = &1
                r = &2;
                return r
            "
        )
    )
}

#[test]
fn assign_equals_function() {
    assert_eq!(
        Ok(Value::Int(2)),
        eval(
            "
                fun foo(): Int {
                    return 1
                }

                fun return2(): Int {
                    return 2
                }

                foo = return2;
                return foo()
            "
        )
    )
}

#[test]
fn assign_add_int() {
    assert_eq!(
        Ok(Value::Int(3)),
        eval(
            "
                var i = 1
                i += 2;
                return i
            "
        )
    )
}

#[test]
fn assign_add_float() {
    assert_delta!(
        0.3,
        eval(
            "
                var i = 0.1
                i += 0.2;
                return i
            "
        )
        .unwrap()
        .unwrap_float(),
        0.0001
    )
}

#[test]
fn assign_add_string() {
    assert_eq!(
        Ok(Value::String("hello world".to_string())),
        eval(
            "
                var s = \"hello\"
                s += \" \";
                s += \"world\";
                return s
            "
        )
    )
}

#[test]
fn assign_divide_int() {
    assert_eq!(
        Ok(Value::Int(5)),
        eval(
            "
                var i = 15
                i /= 3;
                return i
            "
        )
    )
}

#[test]
fn assign_divide_float() {
    assert_eq!(
        Ok(Value::Float(0.5)),
        eval(
            "
                var f = 0.15
                f /= 0.3;
                return f
            "
        )
    )
}

#[test]
fn assign_modulo_int() {
    assert_eq!(
        Ok(Value::Int(4)),
        eval(
            "
                var i = 14
                i %= 10;
                return i
            "
        )
    )
}

#[test]
fn assign_modulo_float() {
    assert_delta!(
        0.4,
        eval(
            "
                var f = 1.4
                f %= 1.0;
                return f
            "
        )
        .unwrap()
        .unwrap_float(),
        0.0001
    )
}

#[test]
fn assign_multiply_int() {
    assert_eq!(
        Ok(Value::Int(15)),
        eval(
            "
                var i = 3
                i *= 5;
                return i
            "
        )
    )
}

#[test]
fn assign_multiply_float() {
    assert_eq!(
        Ok(Value::Float(0.15)),
        eval(
            "
                var i = 0.3
                i *= 0.5;
                return i
            "
        )
    )
}

#[test]
fn assign_subtract_int() {
    assert_eq!(
        Ok(Value::Int(1)),
        eval(
            "
                var i = 3
                i -= 2;
                return i
            "
        )
    )
}

#[test]
fn assign_subtract_float() {
    assert_delta!(
        0.1,
        eval(
            "
                var i = 0.3
                i -= 0.2;
                return i
            "
        )
        .unwrap()
        .unwrap_float(),
        0.0001
    )
}

#[test]
fn assign_and_false_false() {
    assert_eq!(
        Ok(Value::Bool(false)),
        eval(
            "
                var p = false
                p &= false;
                return p
            "
        )
    )
}

#[test]
fn assign_and_false_true() {
    assert_eq!(
        Ok(Value::Bool(false)),
        eval(
            "
                var p = false
                p &= true;
                return p
            "
        )
    )
}

#[test]
fn assign_and_true_false() {
    assert_eq!(
        Ok(Value::Bool(false)),
        eval(
            "
                var p = true
                p &= false;
                return p
            "
        )
    )
}

#[test]
fn assign_and_true_true() {
    assert_eq!(
        Ok(Value::Bool(true)),
        eval(
            "
                var p = true
                p &= true;
                return p
            "
        )
    )
}

#[test]
fn assign_or_false_false() {
    assert_eq!(
        Ok(Value::Bool(false)),
        eval(
            "
                var p = false
                p |= false;
                return p
            "
        )
    )
}

#[test]
fn assign_or_false_true() {
    assert_eq!(
        Ok(Value::Bool(true)),
        eval(
            "
                var p = false
                p |= true;
                return p
            "
        )
    )
}

#[test]
fn assign_or_true_false() {
    assert_eq!(
        Ok(Value::Bool(true)),
        eval(
            "
                var p = true
                p |= false;
                return p
            "
        )
    )
}

#[test]
fn assign_or_true_true() {
    assert_eq!(
        Ok(Value::Bool(true)),
        eval(
            "
                var p = true
                p |= true;
                return p
            "
        )
    )
}

#[test]
fn add_int() {
    assert_eq!(Ok(Value::Int(3)), eval("return 1 + 2"))
}

#[test]
fn add_float() {
    assert_delta!(
        0.3,
        eval("return 0.1 + 0.2").unwrap().unwrap_float(),
        0.0001
    )
}

#[test]
fn add_string() {
    assert_eq!(
        Ok(Value::String("hello world".to_string())),
        eval("return \"hello\" + \" \" + \"world\"")
    )
}

#[test]
fn subtract_int() {
    assert_eq!(Ok(Value::Int(1)), eval("return 3 - 2"))
}

#[test]
fn subtract_float() {
    assert_delta!(
        0.1,
        eval("return 0.3 - 0.1").unwrap().unwrap_float(),
        0.0001
    )
}

#[test]
fn multiply_int() {
    assert_eq!(Ok(Value::Int(15)), eval("return 3*5"))
}

#[test]
fn multiply_float() {
    assert_eq!(Ok(Value::Float(0.15)), eval("return 0.3*0.5"))
}

#[test]
fn divide_int() {
    assert_eq!(Ok(Value::Int(5)), eval("return 15/3"))
}

#[test]
fn divide_float() {
    assert_eq!(Ok(Value::Float(0.5)), eval("return 0.15/0.3"))
}

#[test]
fn modulo_int() {
    assert_eq!(Ok(Value::Int(4)), eval("return 14%10"))
}

#[test]
fn modulo_float() {
    assert_delta!(0.4, eval("return 1.4%1.0").unwrap().unwrap_float(), 0.0001)
}

#[test]
fn and_false_false() {
    assert_eq!(Ok(Value::Bool(false)), eval("return false && false"))
}

#[test]
fn and_false_true() {
    assert_eq!(Ok(Value::Bool(false)), eval("return false && true"))
}

#[test]
fn and_true_false() {
    assert_eq!(Ok(Value::Bool(false)), eval("return true && false"))
}

#[test]
fn and_true_true() {
    assert_eq!(Ok(Value::Bool(true)), eval("return true && true"))
}

#[test]
fn or_false_false() {
    assert_eq!(Ok(Value::Bool(false)), eval("return false || false"))
}

#[test]
fn or_false_true() {
    assert_eq!(Ok(Value::Bool(true)), eval("return false || true"))
}

#[test]
fn or_true_false() {
    assert_eq!(Ok(Value::Bool(true)), eval("return true || false"))
}

#[test]
fn or_true_true() {
    assert_eq!(Ok(Value::Bool(true)), eval("return true || true"))
}

#[test]
fn less_than_int_less_than() {
    assert_eq!(Ok(Value::Bool(true)), eval("return 1 < 2"))
}

#[test]
fn less_than_int_equal() {
    assert_eq!(Ok(Value::Bool(false)), eval("return 2 < 2"))
}

#[test]
fn less_than_int_greater_than() {
    assert_eq!(Ok(Value::Bool(false)), eval("return 3 < 2"))
}

#[test]
fn less_than_float_less_than() {
    assert_eq!(Ok(Value::Bool(true)), eval("return 0.1 < 0.2"))
}

#[test]
fn less_than_float_equal() {
    assert_eq!(Ok(Value::Bool(false)), eval("return 0.2 < 0.2"))
}

#[test]
fn less_than_float_greater_than() {
    assert_eq!(Ok(Value::Bool(false)), eval("return 0.3 < 0.2"))
}

#[test]
fn greater_than_int_less_than() {
    assert_eq!(Ok(Value::Bool(false)), eval("return 1 > 2"))
}

#[test]
fn greater_than_int_equal() {
    assert_eq!(Ok(Value::Bool(false)), eval("return 2 > 2"))
}

#[test]
fn greater_than_int_greater_than() {
    assert_eq!(Ok(Value::Bool(true)), eval("return 3 > 2"))
}

#[test]
fn greater_than_float_less_than() {
    assert_eq!(Ok(Value::Bool(false)), eval("return 0.1 > 0.2"))
}

#[test]
fn greater_than_float_equal() {
    assert_eq!(Ok(Value::Bool(false)), eval("return 0.2 > 0.2"))
}

#[test]
fn greater_than_float_greater_than() {
    assert_eq!(Ok(Value::Bool(true)), eval("return 0.3 > 0.2"))
}

#[test]
fn less_than_or_equal_int_less_than() {
    assert_eq!(Ok(Value::Bool(true)), eval("return 1 <= 2"))
}

#[test]
fn less_than_or_equal_int_equal() {
    assert_eq!(Ok(Value::Bool(true)), eval("return 2 <= 2"))
}

#[test]
fn less_than_or_equal_int_greater_than() {
    assert_eq!(Ok(Value::Bool(false)), eval("return 3 <= 2"))
}

#[test]
fn less_than_or_equal_float_less_than() {
    assert_eq!(Ok(Value::Bool(true)), eval("return 0.1 <= 0.2"))
}

#[test]
fn less_than_or_equal_float_equal() {
    assert_eq!(Ok(Value::Bool(true)), eval("return 0.2 <= 0.2"))
}

#[test]
fn less_than_or_equal_float_greater_than() {
    assert_eq!(Ok(Value::Bool(false)), eval("return 0.3 <= 0.2"))
}

#[test]
fn greater_than_or_equal_int_less_than() {
    assert_eq!(Ok(Value::Bool(false)), eval("return 1 >= 2"))
}

#[test]
fn greater_than_or_equal_int_equal() {
    assert_eq!(Ok(Value::Bool(true)), eval("return 2 >= 2"))
}

#[test]
fn greater_than_or_equal_int_greater_than() {
    assert_eq!(Ok(Value::Bool(true)), eval("return 3 >= 2"))
}

#[test]
fn greater_than_or_equal_float_less_than() {
    assert_eq!(Ok(Value::Bool(false)), eval("return 0.1 >= 0.2"))
}

#[test]
fn greater_than_or_equal_float_equal() {
    assert_eq!(Ok(Value::Bool(true)), eval("return 0.2 >= 0.2"))
}

#[test]
fn greater_than_or_equal_float_greater_than() {
    assert_eq!(Ok(Value::Bool(true)), eval("return 0.3 >= 0.2"))
}

#[test]
fn equals_int_equals() {
    assert_eq!(Ok(Value::Bool(true)), eval("return 1 == 1"))
}

#[test]
fn equals_int_not_equals() {
    assert_eq!(Ok(Value::Bool(false)), eval("return 1 == 2"))
}

#[test]
fn not_equals_int_equals() {
    assert_eq!(Ok(Value::Bool(false)), eval("return 1 != 1"))
}

#[test]
fn not_equals_int_not_equals() {
    assert_eq!(Ok(Value::Bool(true)), eval("return 1 != 2"))
}

#[test]
fn equals_float_equals() {
    assert_eq!(Ok(Value::Bool(true)), eval("return 0.1 == 0.1"))
}

#[test]
fn equals_float_not_equals() {
    assert_eq!(Ok(Value::Bool(false)), eval("return 0.1 == 0.2"))
}

#[test]
fn not_equals_float_equals() {
    assert_eq!(Ok(Value::Bool(false)), eval("return 0.1 != 0.1"))
}

#[test]
fn not_equals_float_not_equals() {
    assert_eq!(Ok(Value::Bool(true)), eval("return 0.1 != 0.2"))
}

#[test]
fn equals_bool_equals() {
    assert_eq!(Ok(Value::Bool(true)), eval("return false == false"))
}

#[test]
fn equals_bool_not_equals() {
    assert_eq!(Ok(Value::Bool(false)), eval("return false == true"))
}

#[test]
fn not_equals_bool_equals() {
    assert_eq!(Ok(Value::Bool(false)), eval("return false != false"))
}

#[test]
fn not_equals_bool_not_equals() {
    assert_eq!(Ok(Value::Bool(true)), eval("return false != true"))
}

#[test]
fn equals_string_equals() {
    assert_eq!(Ok(Value::Bool(true)), eval("return \"hello\" == \"hello\""))
}

#[test]
fn equals_string_not_equals() {
    assert_eq!(
        Ok(Value::Bool(false)),
        eval("return \"hello\" == \"world\"")
    )
}

#[test]
fn not_equals_string_equals() {
    assert_eq!(
        Ok(Value::Bool(false)),
        eval("return \"hello\" != \"hello\"")
    )
}

#[test]
fn not_equals_string_not_equals() {
    assert_eq!(Ok(Value::Bool(true)), eval("return \"hello\" != \"world\""))
}

// -
#[test]
fn equals_list_equals() {
    assert_eq!(Ok(Value::Bool(true)), eval("return [] == []"))
}

#[test]
fn equals_list_not_equals() {
    assert_eq!(Ok(Value::Bool(false)), eval("return [] == [1]"))
}

#[test]
fn not_equals_list_equals() {
    assert_eq!(Ok(Value::Bool(false)), eval("return [] != []"))
}

#[test]
fn not_equals_list_not_equals() {
    assert_eq!(Ok(Value::Bool(true)), eval("return [] != [1]"))
}

#[test]
fn list_mutate_content() {
    assert_eq!(
        Ok(Value::List(vec![Rc::new(RefCell::new(Value::Int(3)))])),
        eval(
            "
                var l = [1]

                l[0] += 2;

                return l
            "
        )
    )
}

#[test]
fn list_mutation_no_side_effects() {
    assert_eq!(
        Ok(Value::Int(1)),
        eval(
            "
                var i = 1
                var l = [i]

                l[0]++;

                return i
            "
        )
    )
}

static FIBONACCI_RECURSIVE_DECLARATIION: &'static str = "
    fun fib(n: Int): Int {
        if n < 2 {
            return n
        }
        return fib(n - 1) + fib(n - 2)
    }
";

#[test]
fn fibonacci_recursive_10() {
    assert_eq!(
        Ok(Value::Int(55)),
        eval(&format!(
            "{FIBONACCI_RECURSIVE_DECLARATIION} return fib(10)",
        ))
    );
}

#[test]
fn fibonacci_recursive_20() {
    assert_eq!(
        Ok(Value::Int(6765)),
        eval(&format!(
            "{FIBONACCI_RECURSIVE_DECLARATIION} return fib(20)",
        ))
    );
}
