pub struct TickitTerm;

extern
{
pub fn tickit_term_new() -> *mut TickitTerm;
pub fn tickit_term_destroy(tt: *mut TickitTerm);
}
