pub type Wasm = i32;

#[derive(Clone, Copy)]
pub struct Pointer(pub u16);

impl TryFrom<i32> for Pointer {
    type Error = String;
    fn try_from(value: i32) -> std::result::Result<Self, Self::Error> {
        u16::try_from(value)
            .map_err(|cause| {
                if value < 0 {
                    format!("Memory address of pointer is negative ({value}). This should never happen.\n  {cause}")
                } else {
                    format!("Unknown error while converting memory address of pointer from WASM's i32 representation ({value}) to u16.\n  {cause}")
                }
            })
            .map(Pointer)
    }
}

impl Into<i32> for Pointer {
    fn into(self) -> i32 {
        i32::from(self.0)
    }
}
