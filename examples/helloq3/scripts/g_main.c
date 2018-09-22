#include "syslib.h"
#include "app.h"

// That's the only function we have to declare outside syslib so vmMain is still the first symbol but we can use panic inside of it
void panic(const char* msg);

/**
 * This is out main function. It should respect the command parameter since that's how we make calls from the host the VM
 */
int vmMain(int command, int arg0, int arg1, int arg2, int arg3, int arg4,
           int arg5, int arg6, int arg7, int arg8, int arg9, int arg10,
           int arg11)
{
    switch(command) {
        case 0:
            test_vm();
            break;
        default:
            panic("Unknown command. VM is confused now");
            break;
    }
        
    return 0;
}
