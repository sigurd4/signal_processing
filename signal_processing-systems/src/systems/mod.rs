moddef::moddef!(
    flat(pub) mod {
        ar,
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