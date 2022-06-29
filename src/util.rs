pub fn or2ro<T, E>(x: Option<Result<T, E>>) -> Result<Option<T>, E> {
    match x {
        Some(Ok(t)) => Ok(Some(t)),
        Some(Err(e)) => Err(e),
        None => Ok(None),
    }
}
