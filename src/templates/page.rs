use horrorshow::prelude::*;

use serde::ser::Serialize;
use serde_json::Serializer;
use super::template::Template;
use shared::config::Config;
use indoc::indoc as js_str;

pub struct Page<C> {
    title: String,
    content: C,
    config: Config
}

impl<C: RenderOnce + Sized> Page<C> {
    pub fn new<T: Into<String>>(title: T, content: C, config: Config)
            -> Self {
        Page {
            title: title.into(),
            content: content,
            config: config
        }
    }
}

impl<C> RenderOnce for Page<C> where C: RenderOnce {
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        let Page {title, content, config} = self;
        let init = InitBuilder::new(config);
        
        tmpl << html! {
            head {
                h1: title;
            }
            body {
                : content;
                : init.to_html();
            }
        };
    }
}

impl<C> Template for Page<C> where C: RenderOnce {}

// helper builder object
struct InitBuilder {
    config: Config,
}

impl InitBuilder {
    fn new(config: Config) -> Self {
        InitBuilder { config }
    }
    
    fn to_html(self) -> Box<RenderBox> {
        box_html! {
            // : self.preload_wasm();
            : self.load_wasm();
            // : self.postload_wasm();
        }
    }
    
    fn gen_config_js(&self) -> String {
        let mut js_str = String::new();
        
        // load the mounter
        js_str.push_str(js_str!("
            import mounter from './mounter.mjs';
        "));
        
        // serialize the config into the js script as json
        js_str.push_str("const config = ");
        let mut utf8_buffer = Vec::new();
        self.config.serialize(&mut Serializer::new(&mut utf8_buffer)).unwrap();
        js_str.push_str(&String::from_utf8(utf8_buffer).unwrap());
        js_str.push_str(";\n");
              
        js_str.push_str(js_str!("
            fetch('frontend.wasm')
            .then(response =>
                response.arrayBuffer()
            )
            .then(bytecode => {
                const wasm = new WebAssembly.Module(bytecode);
                const mounted = mounter();
                const compiled = new WebAssembly.Instance(wasm, mounted.imports);
                const components = mounted.initialize(compiled);
                components.init(config);
            });
        "));
        
        js_str
    }
    
    // fn preload_wasm(&self) -> Box<RenderBox> {
    //     box_html! {
    //         script: Raw("var Module = { wasmBinaryFile: 'frontend.wasm' };");
    //     }
    // }
    
    fn load_wasm<'a>(&'a self) -> Box<RenderBox + 'a> {
        box_html! {
            script(type="module"): Raw(self.gen_config_js())
        }
    }
    
    // fn postload_wasm<'a>(&'a self) -> Box<RenderBox + 'a> {
    //     box_html! {
    //         script: Raw(self.gen_config_js());
    //     }
    // }
}
