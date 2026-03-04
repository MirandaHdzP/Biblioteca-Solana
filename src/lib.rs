use anchor_lang::preluge::*;

declare_id!()

#[program] 
pub mod biblioteca {
    use super::*;
     
    pub fn crear_biblioteca() -> Result<()> {
        
}
}

#[account] 
#[derive(InitSpace)] 
pub struct Biblioteca Tolkien { 
    owner:Pubkey,

    #[max_len(60)] 
    nombre:String,

    #[max_len(10)] 
    libros:Vec<Libro>,   
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)] 
pub struct Libro {
    #[max_len(60)] 
    nombre: String,

    paginas: u8,

    status: bool,
}  
#[derive(Accounts)]   
pub struct NuevaBiblioteca { 
    #[account(mut)]   
    pub owner: Signer<'info>, 

    #[account(
        init,
        payer = owner
        space = Biblioteca::INIT_SPACE + 8,
        seeds = [b"biblioteca", owner.key().as_ref()]  
        bump 
    )]  
    pub biblioteca: Account<'info, Biblioteca>,

    pub system_program: Program<'info, System>,
} 

pub struct NuevoLibro { 
    pub owner: Signer<'info>,

    ##[account(mut)]  
    pub biblioteca: Account<'info, Biblioteca>,
}
