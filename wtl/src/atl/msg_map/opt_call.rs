



//T not limit to sized type
pub struct OptCall<F:?Sized> {
    pub before:Option<Box<F>>,
    pub around:Option<Box<F>>,
    pub after :Option<Box<F>>,
}

impl<F:?Sized> OptCall<F> {
    pub fn new()->OptCall<F>{
        OptCall{
            before:None,
            around:None,
            after :None,
        }
    }
}