#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
use ramhorns_derive::Content;
use ramhorns_ext::Template;
fn main() {
    let v1 = Sa {
        bb: Sb { name: "my_name".to_owned() },
    };
    let s = "
            {{bb.name}}";
    let tpl = Template::new(s).unwrap();
    let rst = tpl.render(&v1);
    {
        ::std::io::_print(format_args!("rst = {0}\n", rst));
    }
}
struct Sa {
    bb: Sb,
}
#[automatically_derived]
impl ::core::fmt::Debug for Sa {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(f, "Sa", "bb", &&self.bb)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for Sa {
    #[inline]
    fn clone(&self) -> Sa {
        Sa {
            bb: ::core::clone::Clone::clone(&self.bb),
        }
    }
}
impl ::ramhorns_ext::Content for Sa {
    #[inline]
    fn capacity_hint(&self, tpl: &::ramhorns_ext::Template) -> usize {
        tpl.capacity_hint() + self.bb.capacity_hint(tpl)
    }
    #[inline]
    fn render_section<C, E, IC>(
        &self,
        section: ::ramhorns_ext::Section<C>,
        encoder: &mut E,
        _content: Option<&IC>,
    ) -> std::result::Result<(), E::Error>
    where
        C: ::ramhorns_ext::traits::ContentSequence,
        E: ::ramhorns_ext::encoding::Encoder,
    {
        section.with(self).render(encoder, Option::<&()>::None)
    }
    #[inline]
    fn render_notnone_section<C, E, IC>(
        &self,
        section: ::ramhorns_ext::Section<C>,
        encoder: &mut E,
        _content: Option<&IC>,
    ) -> std::result::Result<(), E::Error>
    where
        C: ::ramhorns_ext::traits::ContentSequence,
        E: ::ramhorns_ext::encoding::Encoder,
    {
        section.with(self).render(encoder, Option::<&()>::None)
    }
    #[inline]
    fn render_field_escaped<E>(
        &self,
        hash: u64,
        name: &str,
        encoder: &mut E,
    ) -> std::result::Result<bool, E::Error>
    where
        E: ::ramhorns_ext::encoding::Encoder,
    {
        match hash {
            19044748602007918u64 => self.bb.render_escaped(encoder).map(|_| true),
            _ => Ok(false),
        }
    }
    #[inline]
    fn render_field_unescaped<E>(
        &self,
        hash: u64,
        name: &str,
        encoder: &mut E,
    ) -> std::result::Result<bool, E::Error>
    where
        E: ::ramhorns_ext::encoding::Encoder,
    {
        match hash {
            19044748602007918u64 => self.bb.render_unescaped(encoder).map(|_| true),
            _ => Ok(false),
        }
    }
    fn render_field_section<P, E>(
        &self,
        hash: u64,
        name: &str,
        section: ::ramhorns_ext::Section<P>,
        encoder: &mut E,
    ) -> std::result::Result<bool, E::Error>
    where
        P: ::ramhorns_ext::traits::ContentSequence,
        E: ::ramhorns_ext::encoding::Encoder,
    {
        match hash {
            19044748602007918u64 => {
                self.bb
                    .render_section(section, encoder, Option::<&()>::None)
                    .map(|_| true)
            }
            _ => Ok(false),
        }
    }
    fn render_field_inverse<P, E>(
        &self,
        hash: u64,
        name: &str,
        section: ::ramhorns_ext::Section<P>,
        encoder: &mut E,
    ) -> std::result::Result<bool, E::Error>
    where
        P: ::ramhorns_ext::traits::ContentSequence,
        E: ::ramhorns_ext::encoding::Encoder,
    {
        match hash {
            19044748602007918u64 => {
                self.bb
                    .render_inverse(section, encoder, Option::<&()>::None)
                    .map(|_| true)
            }
            _ => Ok(false),
        }
    }
    fn render_field_notnone_section<P, E>(
        &self,
        hash: u64,
        name: &str,
        section: ::ramhorns_ext::Section<P>,
        encoder: &mut E,
    ) -> std::result::Result<bool, E::Error>
    where
        P: ::ramhorns_ext::traits::ContentSequence,
        E: ::ramhorns_ext::encoding::Encoder,
    {
        match hash {
            19044748602007918u64 => {
                self.bb.render_notnone_section(section, encoder, Option::<&()>::None)?;
                Ok(self.bb.is_truthy())
            }
            _ => Ok(false),
        }
    }
}
struct Sb {
    name: String,
}
#[automatically_derived]
impl ::core::fmt::Debug for Sb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(f, "Sb", "name", &&self.name)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for Sb {
    #[inline]
    fn clone(&self) -> Sb {
        Sb {
            name: ::core::clone::Clone::clone(&self.name),
        }
    }
}
impl ::ramhorns_ext::Content for Sb {
    #[inline]
    fn capacity_hint(&self, tpl: &::ramhorns_ext::Template) -> usize {
        tpl.capacity_hint() + self.name.capacity_hint(tpl)
    }
    #[inline]
    fn render_section<C, E, IC>(
        &self,
        section: ::ramhorns_ext::Section<C>,
        encoder: &mut E,
        _content: Option<&IC>,
    ) -> std::result::Result<(), E::Error>
    where
        C: ::ramhorns_ext::traits::ContentSequence,
        E: ::ramhorns_ext::encoding::Encoder,
    {
        section.with(self).render(encoder, Option::<&()>::None)
    }
    #[inline]
    fn render_notnone_section<C, E, IC>(
        &self,
        section: ::ramhorns_ext::Section<C>,
        encoder: &mut E,
        _content: Option<&IC>,
    ) -> std::result::Result<(), E::Error>
    where
        C: ::ramhorns_ext::traits::ContentSequence,
        E: ::ramhorns_ext::encoding::Encoder,
    {
        section.with(self).render(encoder, Option::<&()>::None)
    }
    #[inline]
    fn render_field_escaped<E>(
        &self,
        hash: u64,
        name: &str,
        encoder: &mut E,
    ) -> std::result::Result<bool, E::Error>
    where
        E: ::ramhorns_ext::encoding::Encoder,
    {
        match hash {
            12661497617682247323u64 => self.name.render_escaped(encoder).map(|_| true),
            _ => Ok(false),
        }
    }
    #[inline]
    fn render_field_unescaped<E>(
        &self,
        hash: u64,
        name: &str,
        encoder: &mut E,
    ) -> std::result::Result<bool, E::Error>
    where
        E: ::ramhorns_ext::encoding::Encoder,
    {
        match hash {
            12661497617682247323u64 => self.name.render_unescaped(encoder).map(|_| true),
            _ => Ok(false),
        }
    }
    fn render_field_section<P, E>(
        &self,
        hash: u64,
        name: &str,
        section: ::ramhorns_ext::Section<P>,
        encoder: &mut E,
    ) -> std::result::Result<bool, E::Error>
    where
        P: ::ramhorns_ext::traits::ContentSequence,
        E: ::ramhorns_ext::encoding::Encoder,
    {
        match hash {
            12661497617682247323u64 => {
                self.name
                    .render_section(section, encoder, Option::<&()>::None)
                    .map(|_| true)
            }
            _ => Ok(false),
        }
    }
    fn render_field_inverse<P, E>(
        &self,
        hash: u64,
        name: &str,
        section: ::ramhorns_ext::Section<P>,
        encoder: &mut E,
    ) -> std::result::Result<bool, E::Error>
    where
        P: ::ramhorns_ext::traits::ContentSequence,
        E: ::ramhorns_ext::encoding::Encoder,
    {
        match hash {
            12661497617682247323u64 => {
                self.name
                    .render_inverse(section, encoder, Option::<&()>::None)
                    .map(|_| true)
            }
            _ => Ok(false),
        }
    }
    fn render_field_notnone_section<P, E>(
        &self,
        hash: u64,
        name: &str,
        section: ::ramhorns_ext::Section<P>,
        encoder: &mut E,
    ) -> std::result::Result<bool, E::Error>
    where
        P: ::ramhorns_ext::traits::ContentSequence,
        E: ::ramhorns_ext::encoding::Encoder,
    {
        match hash {
            12661497617682247323u64 => {
                self.name.render_notnone_section(section, encoder, Option::<&()>::None)?;
                Ok(self.name.is_truthy())
            }
            _ => Ok(false),
        }
    }
}
