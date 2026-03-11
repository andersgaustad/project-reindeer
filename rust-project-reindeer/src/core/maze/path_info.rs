use godot::prelude::*;

use crate::core::common::{coordinate::Coordinate, i_generate_mail::IGenerateMail};


#[derive(GodotClass)]
#[class(no_init, base=RefCounted)]
pub struct PathInfo {
    paths : Vec<Vec<Coordinate>>,
    score : usize,

    base : Base<RefCounted>
}


impl PathInfo {
    pub fn new_gd(
        paths : Vec<Vec<Coordinate>>,
        score : usize,

    ) -> Gd<Self> {
        Gd::from_init_fn(|base| {
            Self {
                paths,
                score,
                base,
            }
        })
    }


    pub fn rust_get_paths(&self) -> &Vec<Vec<Coordinate>> {
        &self.paths
    }


    pub fn rust_get_score(&self) -> usize {
        self.score
    }
}


// IGenerateMail

#[godot_dyn]
impl IGenerateMail for PathInfo {
    fn generate_mail(&self) -> GString {
        let PathInfo {
            paths,
            score,
            base : _base
        } = self;

        let n_paths = paths.len();
        if n_paths == 0 {
            let failed_attempt = generate_mail_for_failed_attempt();
            return failed_attempt;
        }

        let n_steps = paths
            .first()
            .map(|path| {
                path.len()
            })
            .unwrap_or(0);

        let number_of_paths_comment = if n_paths > 1 {
            format!("I found multiple paths to the goal - {} to be specific!", n_paths)
        } else {
            "I found a path to the goal!".to_string()
        };

        let mail = format!(
            "\
            Dear [i]Player[/i],\n\
            \n\
            {}\n\
            I took a total of {} steps, and the total cost to get there was {}.\n\
            \n\
            Sincerly,\n\
            - [i]Reindeer[/i]
            ",
            number_of_paths_comment,
            n_steps.checked_sub(1).unwrap_or(0),
            score,
        );

        let gstring = GString::from(&mail);
        gstring
    }
} 


impl IGenerateMail for Option<PathInfo> {
    fn generate_mail(&self) -> GString {
        match self {
            Some(some) => some.generate_mail(),
            None => generate_mail_for_failed_attempt(),
        }
    }
}


impl IGenerateMail for Option<Gd<PathInfo>> {
    fn generate_mail(&self) -> GString {
        match self {
            Some(some) => some.clone().into_dyn().dyn_bind().generate_mail(),
            None => generate_mail_for_failed_attempt(),
        }
    }
}


fn generate_mail_for_failed_attempt() -> GString {
    let mail = 
        "\
        Dear [i]Player[/i],\n\
        \n\
        Due to circumstances out of my control, I was unfortunately not able to find a path to the goal...\n\
        Sorry for the inconvenience.\n\
        \n\
        Sincerly,\n\
        - [i]Reindeer[/i]
        ";
    
    let gstring = GString::from(mail);
    gstring    
}
