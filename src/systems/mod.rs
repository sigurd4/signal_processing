moddef::moddef!(
    flat(pub) mod {
        rpk,
        rtf,
        sos,
        ss,
        tf,
        zpk
    }
);

pub enum IrType
{
    FIR,
    IIR
}