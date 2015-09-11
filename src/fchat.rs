use ticket;

pub struct FChat {
    tmp: ()
}

impl FChat {
    fn new(ticket: ticket::Ticket, character: &str) -> FChat {
        FChat {
            tmp: ()
        }
    }
}
