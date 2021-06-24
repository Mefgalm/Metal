pub fn combine_errors<T: Clone, E: Clone>(errors: &Vec<Result<T, E>>) -> Result<Vec<T>, Vec<E>> {
    let mut oks = vec![];
    let mut errs = vec![];
    for error in errors {
        match error {
            Ok(value) => oks.push(value.clone()),
            Err(err) => errs.push(err.clone())
        }
    }
    if errs.is_empty() {
        Ok(oks)
    } else {
        Err(errs)
    }
}
