use crate::components::boo;
use leptos::prelude::*;
use leptos_mview::mview;

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
    let frame = FRAME.lines().collect::<Vec<_>>();
    mview! {
        main (
            h2 ("Boo")
            boo::Terminal
                frame={frame}
                whitespace_padding={15}
                columns={100}
                rows={41};

        )
    }
}
