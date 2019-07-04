use crate::vm::machine::Machine;
use crate::vm::ErrorType;
use crate::vm::Value;

pub fn add(machine: &mut Machine) -> Result<(), ErrorType> {
    let a = match machine.pop() {
        Some(n) => n,
        None => return Err(ErrorType::StackUnderflow),
    };
    let b = match machine.pop() {
        Some(n) => n,
        None => return Err(ErrorType::StackUnderflow)
    };
    machine.push(a + b);
    Ok(())
}

pub fn sub(machine: &mut Machine) -> Result<(), ErrorType> {
    let a = match machine.pop() {
        Some(n) => n,
        None => return Err(ErrorType::StackUnderflow)
    };
    let b = match machine.pop() {
        Some(n) => n,
        None => return Err(ErrorType::StackUnderflow)
    };
    machine.push(a - b);
    Ok(())
}

pub fn mult(machine: &mut Machine) -> Result<(), ErrorType> {
    let a = match machine.pop() {
        Some(n) => n,
        None => return Err(ErrorType::StackUnderflow)
    };
    let b = match machine.pop() {
        Some(n) => n,
        None => return Err(ErrorType::StackUnderflow)
    };
    machine.push(a * b);
    Ok(())
}

pub fn div(machine: &mut Machine) -> Result<(), ErrorType> {
    let a = match machine.pop() {
        Some(n) => n,
        None => return Err(ErrorType::StackUnderflow)
    };
    let b = match machine.pop() {
        Some(n) => n,
        None => return Err(ErrorType::StackUnderflow)
    };
    machine.push(a / b);
    Ok(())
}

pub fn dup(machine: &mut Machine) -> Result<(), ErrorType> {
    let a = match machine.pop() {
        Some(n) => n,
        None => return Err(ErrorType::StackUnderflow)
    };
    machine.push(a);
    machine.push(a);
    Ok(())
}

pub fn drop(machine: &mut Machine) -> Result<(), ErrorType> {
    let _a = match machine.pop() {
        Some(n) => n,
        None => return Err(ErrorType::StackUnderflow)
    };
    Ok(())
}

pub fn swap(machine: &mut Machine) -> Result<(), ErrorType> {
    let a = match machine.pop() {
        Some(n) => n,
        None => return Err(ErrorType::StackUnderflow)
    };
    let b = match machine.pop() {
        Some(n) => n,
        None => return Err(ErrorType::StackUnderflow)
    };
    machine.push(a);
    machine.push(b);
    Ok(())
}

pub fn over(machine: &mut Machine) -> Result<(), ErrorType> {
    let a = match machine.pop() {
        Some(n) => n,
        None => return Err(ErrorType::StackUnderflow)
    };
    let b = match machine.pop() {
        Some(n) => n,
        None => return Err(ErrorType::StackUnderflow)
    };
    machine.push(b);
    machine.push(a);
    machine.push(b);
    Ok(())
}

pub fn rot(machine: &mut Machine) -> Result<(), ErrorType> {
    let a = match machine.pop() {
        Some(n) => n,
        None => return Err(ErrorType::StackUnderflow)
    };
    let b = match machine.pop() {
        Some(n) => n,
        None => return Err(ErrorType::StackUnderflow)
    };
    let c = match machine.pop() {
        Some(n) => n,
        None => return Err(ErrorType::StackUnderflow)
    };
    machine.push(b);
    machine.push(a);
    machine.push(c);
    Ok(())
}

pub fn dot(machine: &mut Machine) -> Result<(), ErrorType> {
    let a = match machine.pop() {
        Some(n) => n,
        None => return Err(ErrorType::StackUnderflow)
    };
    println!("{}", a);
    Ok(())
}

pub fn sdot(machine: &mut Machine) -> Result<(), ErrorType> {
    println!("{:?}", machine.stack);
    Ok(())
}

pub fn eq(machine: &mut Machine) -> Result<(), ErrorType> {
    let a = match machine.pop() {
        Some(n) => n,
        None => return Err(ErrorType::StackUnderflow)
    };
    let b = match machine.pop() {
        Some(n) => n,
        None => return Err(ErrorType::StackUnderflow)
    };

    if a == b {
        machine.push(-1);
    } else {
        machine.push(0);
    }

    Ok(())
}

pub fn greater_than(machine: &mut Machine) -> Result<(), ErrorType> {
    let a = match machine.pop() {
        Some(n) => n,
        None => return Err(ErrorType::StackUnderflow)
    };
    let b = match machine.pop() {
        Some(n) => n,
        None => return Err(ErrorType::StackUnderflow)
    };

    if b > a {
        machine.push(-1);
    } else {
        machine.push(0);
    }

    Ok(())
}

pub fn less_than(machine: &mut Machine) -> Result<(), ErrorType> {
    let a = match machine.pop() {
        Some(n) => n,
        None => return Err(ErrorType::StackUnderflow)
    };
    let b = match machine.pop() {
        Some(n) => n,
        None => return Err(ErrorType::StackUnderflow)
    };

    if b < a {
        machine.push(-1);
    } else {
        machine.push(0);
    }

    Ok(())
}

pub fn and(machine: &mut Machine) -> Result<(), ErrorType> {
    let a = match machine.pop() {
        Some(n) => n,
        None => return Err(ErrorType::StackUnderflow)
    };
    let b = match machine.pop() {
        Some(n) => n,
        None => return Err(ErrorType::StackUnderflow)
    };

    machine.push(a & b);

    Ok(())
}

pub fn or(machine: &mut Machine) -> Result<(), ErrorType> {
    let a = match machine.pop() {
        Some(n) => n,
        None => return Err(ErrorType::StackUnderflow)
    };
    let b = match machine.pop() {
        Some(n) => n,
        None => return Err(ErrorType::StackUnderflow)
    };

    machine.push(a | b);

    Ok(())
}

pub fn invert(machine: &mut Machine) -> Result<(), ErrorType> {
    let a = match machine.pop() {
        Some(n) => n,
        None => return Err(ErrorType::StackUnderflow)
    };

    machine.push(!a);

    Ok(())
}

pub fn clearstack(machine: &mut Machine) -> Result<(), ErrorType> {
    machine.stack.clear();
    Ok(())
}

pub fn branch0(machine: &mut Machine) -> Result<(), ErrorType> {
    let a = match machine.pop() {
        Some(n) => n,
        None => return Err(ErrorType::StackUnderflow)
    };

    if a != 0 {
        machine.sp += 1;
        return Ok(());
    }

    if machine.sp >= machine.data.len() {
        return Err(ErrorType::BranchOutOfBounds);
    }

    if let Value::Number(n) = machine.data[machine.sp] {
        machine.sp += n as usize;
    } else {
        return Err(ErrorType::InvalidOffset);
    }

    Ok(())
}

pub fn branch(machine: &mut Machine) -> Result<(), ErrorType> {
    if machine.sp >= machine.data.len() {
        return Err(ErrorType::BranchOutOfBounds);
    }

    if let Value::Number(n) = machine.data[machine.sp] {
        machine.sp += n as usize;
    } else {
        return Err(ErrorType::InvalidOffset);
    }

    Ok(())
}

pub fn to_r(machine: &mut Machine) -> Result<(), ErrorType> {
    let a = match machine.pop() {
        Some(n) => n,
        None => return Err(ErrorType::StackUnderflow)
    };

    machine.return_stack.push(a as usize);
    Ok(())
}

pub fn from_r(machine: &mut Machine) -> Result<(), ErrorType> {
    let a = match machine.return_stack.pop() {
        Some(n) => n,
        None => return Err(ErrorType::StackUnderflow)
    };

    machine.push(a as i32);
    Ok(())
}

pub fn if_(machine: &mut Machine) -> Result<(), ErrorType> {
    let a = match machine.pop() {
        Some(n) => n,
        None => return Err(ErrorType::StackUnderflow)
    };

    if a != 0 {
        return Ok(());
    }

    let mut ifs = 0;
    while machine.sp < machine.data.len() {
        if let Value::Word(w) = &machine.data[machine.sp] {
            if w == "then" && ifs == 0 {
                return Ok(());
            } else if w == "else" && ifs == 0 {
                machine.control_flow_stack.push(0);
                return Ok(());
            } else if w == "if" {
                ifs += 1;
                machine.sp += 1;
            } else if w == "then" && ifs != 0 {
                ifs -= 1;
                machine.sp += 1;
            } else {
                machine.sp += 1;
            }
        } else {
            machine.sp += 1;
        }
    }

    Err(ErrorType::UnbalancedControl)
}

pub fn then(_machine: &mut Machine) -> Result<(), ErrorType> {
    Ok(())
}

pub fn else_(machine: &mut Machine) -> Result<(), ErrorType> {
    let a = match machine.control_flow_stack.pop() {
        Some(n) => n,
        None => 1,
    };

    if a == 0 {
        return Ok(());
    }

    let mut ifs = 0;
    while machine.sp < machine.data.len() {
        if let Value::Word(w) = &machine.data[machine.sp] {
            if w == "then" && ifs == 0 {
                return Ok(());
            } else if w == "if" {
                ifs += 1;
                machine.sp += 1;
            } else if w == "then" && ifs != 0 {
                ifs -= 1;
                machine.sp += 1;
            } else {
                machine.sp += 1;
            }
        } else {
            machine.sp += 1;
        }
    }

    Err(ErrorType::UnbalancedControl)
}
