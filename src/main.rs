//mod compound;
//mod scalar;
//mod signedintegers;
//mod unsigned;
//mod floating;
//mod typeinterface;
mod listmanager;
//mod  turbofish;
mod calculator;
mod bankerror;

mod word;
pub fn main() {
    bankerror::run();
    calculator::run();
    listmanager::run();
    word::run();
     //urbofish::run();
    //typeinterface::run();
    //floating::run();
    //unsigned::run();
     //signedintegers::run();
    //compound::run();
    //scalar::run();


}
