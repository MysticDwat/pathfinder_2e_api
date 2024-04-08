// enum to handle check results' degree of success
#[derive(Debug)]
pub enum SuccessDegree {
    CriticalSuccess,
    Success,
    Failure,
    CriticalFailure
}