// enum to handle check results' degree of success
#[derive(Debug)]
pub enum CheckResult {
    CriticalSuccess,
    Success,
    Failure,
    CriticalFailure
}