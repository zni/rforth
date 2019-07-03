use crate::vm::machine::Machine;
use crate::vm::ErrorType;

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

// TODO
pub fn branch0(machine: &mut Machine) -> Result<(), ErrorType> {
    let a = match machine.pop() {
        Some(n) => n,
        None => return Err(ErrorType::StackUnderflow)
    };

    Ok(())
}
