use anchor_lang::prelude::*;

declare_id!("EZz9VjY1SR1ZKDUA4VmipqUjTns73y7ig9BKxicUJVy4");

#[program]
pub mod biblioteca_tolkien {
    use super::*;

    pub fn crear_biblioteca(context: Context<NuevaBiblioteca>, nombre: String) -> Result<()> {
        let owner: Pubkey = context.accounts.owner.key();
        let libros: Vec<Libro> = Vec::new();

        context.accounts.biblioteca.set_inner(BibliotecaTolkien {
            owner,
            nombre,
            libros,
        });

        Ok(())
    }

    pub fn agregar_libro(
        context: Context<NuevoLibro>,
        nombre: String,
        paginas: u16,
        editorial: String,
        formato: String,
    ) -> Result<()> {

        let libro = Libro {
            nombre,
            paginas,
            editorial,
            status: false,
            formato,
        };

        context.accounts.biblioteca.libros.push(libro);

        Ok(())
    }

    pub fn ver_libros(context: Context<NuevoLibro>) -> Result<()> {
        msg!(
            "La lista de libros es: {:#?}",
            context.accounts.biblioteca.libros
        );

        Ok(())
    }

    pub fn eliminar_libro(context: Context<NuevoLibro>, nombre: String) -> Result<()> {
        let libros = &mut context.accounts.biblioteca.libros;

        for i in 0..libros.len() {
            if libros[i].nombre == nombre {
                libros.remove(i);
                msg!("Libro {} eliminado!", nombre);
                break;
            }
        }

        Ok(())
    }

    pub fn alternar_estado(context: Context<NuevoLibro>, nombre: String) -> Result<()> {
        let libros = &mut context.accounts.biblioteca.libros;

        for i in 0..libros.len() {
            if libros[i].nombre == nombre {
                let estado_actual = libros[i].status;
                libros[i].status = !estado_actual;

                msg!(
                    "El libro {} ahora tiene estado de lectura: {}",
                    nombre,
                    libros[i].status
                );

                break;
            }
        }

        Ok(())
    }
}



#[account]
#[derive(InitSpace)]
pub struct BibliotecaTolkien {
    pub owner: Pubkey,

    #[max_len(60)]
    pub nombre: String,

    #[max_len(10)]
    pub libros: Vec<Libro>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Libro {

    #[max_len(60)]
    pub nombre: String,

    pub paginas: u16,

    #[max_len(30)]
    pub editorial: String,

    pub status: bool, // true = leído, false = pendiente

    #[max_len(10)]
    pub formato: String, // fisico o epub
}

#[derive(Accounts)]
pub struct NuevaBiblioteca<'info> {

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = BibliotecaTolkien::INIT_SPACE + 8,
        seeds = [b"biblioteca", owner.key().as_ref()],
        bump
    )]
    pub biblioteca: Account<'info, BibliotecaTolkien>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct NuevoLibro<'info> {

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut)]
    pub biblioteca: Account<'info, BibliotecaTolkien>,
}

