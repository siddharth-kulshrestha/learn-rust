//mod network {
    //fn connect() {
        
    //}
    
    //mod server {
        //fn connect() {
        
        //}
    //} 
//// In this scenario, we are creating server as submodule of network, which makes more sense here.
//}
// can be used as network::connect()
//

 //mod client {
 //fn connect() { }
//} // can be a sibling to network as well.

// Lets move all the modules in separate files
// here we need to just declare it.
// by default all modules in rust are private to make them public we need to add pub keyword
pub mod client;
pub mod network;


