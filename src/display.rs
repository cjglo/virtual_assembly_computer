// use std::process::Command;


pub fn clear_screen()
{
    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    println!(".\nBeginning VAC in Terminal\n.\n.");
    
}

pub fn window(last_command: &str, mioi: &super::memory_io_interface::MemIOInterface, error: u8)
{   
    let error_line = match error
    {
        0 => "Success                              ",
        1 => "!!! Entered a Non-Implemented Command",
        2 => "!!! Null Command (No Command Found)  ",
        3 => "!!! Register or Arguments Not Found  ",
        4 => "!!! DR or Arguments Were Not Numeric ",
        _ => "                                     ",
    };


    println!("\n\n\n\n\n");
    println!("################################################################################");
    println!("################################################################################");
    println!("# Virtual Assebmly Computer - v0.1                                             #");
    println!("#                                                                              #");
    println!("#==============================================================================#");
    println!("# R0:{0:10b} ||  R1: {1:10b} || R2:{2:10b}    ||                      #", mioi.give_registers(0), mioi.give_registers(1), mioi.give_registers(2));
    println!("# R3:{0:10b} ||  R4: {1:10b} ||  R5:{2:10b}   ||                     #", mioi.give_registers(3), mioi.give_registers(4), mioi.give_registers(5));
    println!("# R6:{0:10b} ||  R7: {1:10b} ||  R8:{2:10b}   ||                      #", mioi.give_registers(6), mioi.give_registers(7), mioi.give_registers(8));
    println!("# R9:{0:10b} ||                                                             #", mioi.give_registers(9));
    println!("#==============================================================================#");
    println!("#  Registers above, Assembly Commands in Manual (will be impl on command)      #");
    println!("#                                                                              #");
    println!("#==> Last Command: {}                                    ", last_command);
    println!("#==> Result: {}                           #", error_line);
    println!("#                                                                              #");
    println!("#  Current Instruction Address: {}                #", "(Not Implemented, No File Read)");
    println!("#  Type Assembly Command Below...                                              #");
    println!("################################################################################");
    println!("################################################################################");
}