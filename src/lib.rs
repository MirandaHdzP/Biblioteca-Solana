use anchor_lang::prelude::*; 

declare_id!("76PEDV84KFQ9VafGwGXmbhT62rTmDXV1whyDzcU94vfU");

#[program]
pub mod biblioteca {
    use super::*;

    pub fn crear_biblioteca(context: Context<NuevaBiblioteca>, nombre: String) -> Result<()> { 
        let owner: Pubkey = contex.accounts.owner.key(); 
        let libros: Vec<Libro> = Vec::new(); 
        
        context.accounts.biblioteca.set_inner(Biblioteca {
            owner,
            nombre,
            libros,
         });

        Ok(())
    }

    pub fn agregar_libro(context: Context<NuevoLibro>, nombre: String, paginas: u16) -> Result<()> {
        let libro = Libro { 
            nombre,
            paginas,
            disponible: true, 
    };

    context.accounts.biblioteca.libros.push(libro);
    Ok(())

    }   

    pub fn eliminar_libro(context: Context<NuevoLibro>) -> Result<()> {
        let libros = context.accounts.biblioteca.libros; 
        msg!("La lista de libros es: {:#?}", libros);
        Ok(())
    }

    pub fn ver_libro() -> Result<()> {
    }

    pub fn alternar_estado() -> Result<()> {
    }


#[account] 
#[derive(InitSpace)] 
pub struct BibliotecaTolkien { 
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

    paginas: u16,

    disponible: bool, 
}  
#[derive(Accounts)]   
pub struct NuevaBiblioteca { 
    #[account(mut)]   
    pub owner: Signer<'info>, 

    #[account(
        init,
        payer = owner,
        space = BibliotecaTolkien::INIT_SPACE + 8,
        seeds = [b"biblioteca", owner.key().as_ref()]  
        bump 
    )]  
    pub biblioteca: Account<'info, BibliotecaTolkien>,

    pub system_program: Program<'info, System>,
} 


#[derive(Accounts)]  
pub struct NuevoLibro<'info>, { 
    pub owner: Signer<'info>,

    #[account(mut)]  
    pub biblioteca: Account<'info, BibliotecaTolkien>,
    
}
