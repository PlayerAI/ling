use std::cell::Cell;
use std::rc::Rc;

use num_bigint::BigInt;

pub(crate) struct Heap {
    used: Rc<Cell<u64>>,
    limit: u64,
}

impl Heap {
    pub(crate) fn new(limit: u64) -> Self {
        Self {
            used: Rc::new(Cell::new(0)),
            limit,
        }
    }

    pub(crate) fn can_allocate(&self, bytes: u64) -> bool {
        self.used
            .get()
            .checked_add(bytes)
            .is_some_and(|next| next <= self.limit)
    }

    pub(crate) fn int(&self, value: BigInt) -> Result<Value, ()> {
        let bytes = value.bits().saturating_add(7) / 8;
        self.allocate(value, bytes).map(Value::Int)
    }

    pub(crate) fn text(&self, value: String) -> Result<Value, ()> {
        let bytes = u64::try_from(value.len()).map_err(|_| ())?;
        self.allocate(value, bytes).map(Value::Text)
    }

    pub(crate) fn closure(&self, function: u32, bound: Vec<BoundValue>) -> Result<Value, ()> {
        const CLOSURE_BASE_BYTES: u64 = 16;
        const BOUND_VALUE_BYTES: u64 = 16;

        let count = u64::try_from(bound.len()).map_err(|_| ())?;
        let bytes = count
            .checked_mul(BOUND_VALUE_BYTES)
            .and_then(|bytes| bytes.checked_add(CLOSURE_BASE_BYTES))
            .ok_or(())?;
        self.allocate(Closure { function, bound }, bytes)
            .map(Value::Closure)
    }

    fn allocate<T>(&self, value: T, bytes: u64) -> Result<Rc<Allocation<T>>, ()> {
        let next = self.used.get().checked_add(bytes).ok_or(())?;
        if next > self.limit {
            return Err(());
        }
        self.used.set(next);
        Ok(Rc::new(Allocation {
            value,
            bytes,
            used: Rc::clone(&self.used),
        }))
    }
}

#[derive(Clone)]
pub(crate) enum BoundValue {
    Value(Value),
    SelfReference,
}

pub(crate) struct Closure {
    function: u32,
    bound: Vec<BoundValue>,
}

impl Closure {
    pub(crate) const fn function(&self) -> u32 {
        self.function
    }

    pub(crate) fn bound(&self) -> &[BoundValue] {
        &self.bound
    }
}

pub(crate) struct Allocation<T> {
    value: T,
    bytes: u64,
    used: Rc<Cell<u64>>,
}

impl<T> Allocation<T> {
    pub(crate) const fn value(&self) -> &T {
        &self.value
    }
}

impl<T> Drop for Allocation<T> {
    fn drop(&mut self) {
        self.used.set(self.used.get().saturating_sub(self.bytes));
    }
}

#[derive(Clone)]
pub(crate) enum Value {
    Unit,
    Bool(bool),
    Int(Rc<Allocation<BigInt>>),
    Text(Rc<Allocation<String>>),
    Closure(Rc<Allocation<Closure>>),
}

impl Value {
    pub(crate) const fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(value) => Some(*value),
            Self::Unit | Self::Int(_) | Self::Text(_) | Self::Closure(_) => None,
        }
    }

    pub(crate) fn as_int(&self) -> Option<&BigInt> {
        match self {
            Self::Int(value) => Some(value.value()),
            Self::Unit | Self::Bool(_) | Self::Text(_) | Self::Closure(_) => None,
        }
    }

    pub(crate) fn as_text(&self) -> Option<&str> {
        match self {
            Self::Text(value) => Some(value.value()),
            Self::Unit | Self::Bool(_) | Self::Int(_) | Self::Closure(_) => None,
        }
    }

    pub(crate) fn as_closure(&self) -> Option<&Rc<Allocation<Closure>>> {
        match self {
            Self::Closure(value) => Some(value),
            Self::Unit | Self::Bool(_) | Self::Int(_) | Self::Text(_) => None,
        }
    }

    pub(crate) const fn is_unit(&self) -> bool {
        matches!(self, Self::Unit)
    }
}
