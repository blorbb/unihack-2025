use crate::components::boo;
use leptos::prelude::*;
use leptos_mview::mview;
use serde::{Deserialize, Serialize};

const FRAME: &'static str = r#"                                                                                                    
                                                                                                    
                                      <span class="b">+++==*%%%%%%%%%%%%*==+++</span>                                      
                                  <span class="b">++****++</span>                <span class="b">++****++</span>                                  
                              <span class="b">++**++</span>                            <span class="b">++**++</span>                              
                          <span class="b">xx**+=</span>          o+*%$@@@@@@$%*+o          <span class="b">=+**xx</span>                          
                        <span class="b">xx**oo</span>      ·=$@@@@@@@$$$$$$$$@@@@@@@$=·      <span class="b">oo**xx</span>                        
                      <span class="b">xx**</span>       x$@@@$$$$$$$$$$$$$$$$$$$$$$$$@@@$x       <span class="b">**xx</span>                      
                    <span class="b">ox**</span>      ·$@@$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$@@$·      <span class="b">**xo</span>                    
                    <span class="b">==+~</span>    ~@@@$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$@@@~    <span class="b">~+==</span>                    
                  <span class="b">x+++</span>     $@$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$@$     <span class="b">+++x</span>                  
                  <span class="b">==</span>     ·@@$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$@@·     <span class="b">==</span>                  
                <span class="b">ox++</span>    ~@$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$@~    <span class="b">++xo</span>                
                <span class="b">+++~</span>    @$$$$$@@@@@@@@@$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$@    <span class="b">~+++</span>                
                <span class="b">==</span>     $$$$$@@%%%%%%$$$$$$$@@@@@$$$@@@@@@@@@@@@@@@@@@@@$$$$$$     <span class="b">==</span>                
                <span class="b">==</span>     @$$$$*                  $$$$%                  =$$$$$@     <span class="b">==</span>                
                <span class="b">==</span>    ·$$$$@                   x@$@                    @$$$$$·    <span class="b">==</span>                
                <span class="b">==</span>    ·@$$$$%                 ·$$$$%                  *$$$$$@·    <span class="b">==</span>                
                <span class="b">==</span>    ·@$$$$@@$%%$$$$$$@@@@@@@@$$$$@@@@@@@@@@@@@@@@@@@@$$$$$@·    <span class="b">==</span>                
                <span class="b">==</span>    ·@$$$$$$$@@@@@$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$@·    <span class="b">==</span>                
                <span class="b">==</span>    ·@$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$@·    <span class="b">==</span>                
                <span class="b">==</span>    ·@$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$@·    <span class="b">==</span>                
                <span class="b">==</span>    ·@$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$@·    <span class="b">==</span>                
                <span class="b">==</span>    ·@$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$@·    <span class="b">==</span>                
                <span class="b">==</span>    ·@$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$@·    <span class="b">==</span>                
                <span class="b">==</span>    ·@$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$@·    <span class="b">==</span>                
                <span class="b">==</span>    ·@$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$@·    <span class="b">==</span>                
                <span class="b">==</span>    ·@$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$@·    <span class="b">==</span>                
                <span class="b">==</span>    ·@$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$@·    <span class="b">==</span>                
                <span class="b">==</span>    ·@$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$@·    <span class="b">==</span>                
                <span class="b">==</span>    ·@$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$@·    <span class="b">==</span>                
                <span class="b">==</span>    ~$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$~    <span class="b">==</span>                
                <span class="b">==</span>     @@$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$@@     <span class="b">==</span>                
                <span class="b">==x·</span>    $@@$$$$$$$$$@@@@@@@@@@$$$$$$$$@@@@@@@@@@$$$$$$$$$@@$    <span class="b">·+==</span>                
                <span class="b">++++</span>      =@@@@@@@@@*       x$@@@@@@@@$x       *@@@@@@@@@=      <span class="b">++++</span>                
                <span class="b">xx==++</span>                                                        <span class="b">++==oo</span>                
                  <span class="b">++===+</span>              <span class="b">++%%+o</span>            <span class="b">o+%%++</span>              <span class="b">+===++</span>                  
                    <span class="b">++=====%+=++++*=*========***++++***========*=*++++=+%=====++</span>                    
                      <span class="b">xx++==******====++</span>  <span class="b">++==********==++</span>  <span class="b">++====******==++xx</span>                      
                              <span class="b">++++</span>              <span class="b">++++</span>              <span class="b">++++</span>                              
                                                                                                    "#;

stylance::import_style!(s, "boo.module.scss");
#[component]
pub fn BooPage() -> impl IntoView {
    let frames_resource = OnceResource::new(crate::api::get_animations());
    mview! {
        main (
        Suspense
            fallback=[mview! { p("Loading group...") }]
        (
            ErrorBoundary
                fallback={|err| mview! { "Oops!" f["{:#?}", err()] }}
            (

                [Suspend::new(async move {
                    let frames = frames_resource.get();
                    let view = match frames {
                        Some(Ok(v)) => mview! {

                            boo::Terminal
                                frames={v}
                                whitespace_padding={15}
                                columns={100}
                                rows={41};

                        },
                        _ => return Err(GetError::ServerError)
                    };
                    Ok(view)
                })]
            )
        )
        )
    }
}
#[derive(thiserror::Error, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GetError {
    #[error("Invalid group ID.")]
    InvalidId,
    #[error("Group not found.")]
    GroupNotFound,
    #[error("Server error.")]
    ServerError,
}
