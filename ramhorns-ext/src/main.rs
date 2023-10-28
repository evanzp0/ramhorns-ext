
use ramhorns_derive::Content;
use ramhorns_ext::Template;
fn main() {
    let v1 = Sa {
        bb: Sb {
            name: "my_name".to_owned(),
            prop1: vec![1, 2],
        },
    };

    let s = "
        {{?bb prop1}}
            {{#bb prop1}}
                {{$value}} a,
            {{/bb prop1}}
        {{/bb prop1}}
    ";

    // let s = "
    // {{?bb}}
    //     {{?prop1}}
    //         {{#bb}}
    //             {{#prop1}}
    //                 {{$value}} a,
    //             {{/prop1}}
    //         {{/bb}}
    //     {{/prop1}}
    // {{/bb}}
    // ";

    let tpl = Template::new(s).unwrap();
    println!("tpl = {:?}", tpl);
    let rst = tpl.render(&v1);
    println!("rst = {}", rst)
}

#[derive(Content)]
struct Sa {
    bb: Sb,
}

#[derive(Content)]
struct Sb {
    name: String,
    prop1: Vec<u8>,
}
