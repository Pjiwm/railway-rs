use std::{borrow::Borrow, marker::PhantomData};

pub struct Railway<I, T, E>
where
    I: Fn(Result<T, E>) -> Result<T, E>,
{
    _phantom_err: PhantomData<E>,
    _phantom_type: PhantomData<T>,
    function: I,
}
impl<I, T, E> Railway<I, T, E>
where
    I: Fn(Result<T, E>) -> Result<T, E>,
{
    pub fn new(init: I) -> Self {
        Railway {
            _phantom_err: PhantomData,
            _phantom_type: PhantomData,
            function: init,
        }
    }

    pub fn connect<'a, F>(
        self,
        function: &'a F,
    ) -> Railway<impl Fn(Result<T, E>) -> Result<T, E> + 'a, T, E>
    where
        I: 'a,
        F: Fn(T) -> Result<T, E> + 'a,
    {
        let connected = move |x| connect(function, x);
        Railway {
            _phantom_err: PhantomData,
            _phantom_type: PhantomData,
            function: move |x| connected(self.function.borrow()(x)),
        }
    }

    pub fn recover<'a, F>(
        self,
        function: &'a F,
    ) -> Railway<impl Fn(Result<T, E>) -> Result<T, E> + 'a, T, E>
    where
        I: 'a,
        F: Fn(E) -> Result<T, E> + 'a,
    {
        let connected = move |x| recover(function, x);
        Railway {
            _phantom_err: PhantomData,
            _phantom_type: PhantomData,
            function: move |x| connected(self.function.borrow()(x)),
        }
    }

    pub fn call(&self, input: Result<T, E>) -> Result<T, E> {
        (self.function)(input)
    }

    pub fn extend<'a, F>(
        self,
        railway: Railway<F, T, E>,
    ) -> Railway<impl Fn(Result<T, E>) -> Result<T, E> + 'a, T, E>
    where
        I: 'a,
        F: Fn(Result<T, E>) -> Result<T, E> + 'a,
    {
        let function = railway.function;
        Railway {
            _phantom_err: PhantomData,
            _phantom_type: PhantomData,
            function: move |x| function(self.function.borrow()(x)),
        }
    }
}

fn connect<F, T, E>(f: F, input: Result<T, E>) -> Result<T, E>
where
    F: Fn(T) -> Result<T, E>,
{
    match input {
        Ok(x) => f(x),
        Err(e) => Err(e),
    }
}

fn recover<F, T, E>(f: F, input: Result<T, E>) -> Result<T, E>
where
    F: Fn(E) -> Result<T, E>,
{
    match input {
        Ok(x) => Ok(x),
        Err(e) => f(e),
    }
}
