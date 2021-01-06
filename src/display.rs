use std::process::Command;



pub fn clear_screen()
{
    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    println!(".\nBeginning VAC\n.\n.");
    
}

pub fn window(last_command: &str, reg: &super::memory_io_interface::MemIOInterface)
{
    println!("################################################################################");
    println!("################################################################################");
    println!("# Virtual Assebmly Computer - v0.1                                             #");
    println!("#                                                                              #");
    println!("#==============================================================================#");
    println!("# R1:{0:#10b} ||  R2: {1:#10b} || R3:{2:#10b}    ||                      #", mioi.mem[0].val, 10, 255);
    println!("# R4:{0:#10b} ||  R5: {1:#10b} ||  R6:{2:#10b}    ||                     #", 0, 2, 7);
    println!("# R7:{0:#10b} ||  R8: {1:#10b} ||  R9:{2:#10b}   ||                      #", 1, 2, 203);
    println!("# R10:{0:#10b} ||                                                             #", 80);
    println!("#==============================================================================#");
    println!("#  Registers above, Assembly Commands in Manual (will be impl on command)      #");
    println!("#                                                                              #");
    println!("#  Last Command: {}                                      ", last_command);
    println!("#                                                                              #");
    println!("#  Current Instruction Address: {}                #", "(Not Implemented, No File Read)");
    println!("#  Type Assembly Command Below...                                              #");
    println!("################################################################################");
    println!("################################################################################");
}