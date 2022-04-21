#include <stdio.h>
#include <string.h>
#include <stdlib.h>

int main(int argc, char *argv[])
{

       // Reserve 5 byte of buffer plus the terminating NULL.
       // should allocate 8 bytes = 2 double words,
       // To overflow, need more than 8 bytes...
       char buffer[10]; // If more than 8 characters input
                        // by user, there will be access
                        // violation, segmentation fault
       int password = 0;

       // a prompt how to execute the program...
       if (argc < 2)
       {
              printf("strcpy() NOT executed....\n");
              printf("Syntax: %s <characters>\n", argv[0]);
              exit(0);
       }

       // copy the user input to mybuffer, without any
       // bound checking a secure version is srtcpy_s()

       strcpy(buffer, argv[1]);

       if (strcmp(buffer, "mypassword"))
       {
              printf("Password is wrong\n\r");
       }
       else
       {
              printf("Password is correct\n\r");
              password = 1;
       }

       if (password)
       {
              printf("Correct password entered, secret information accessed!");
       }

       return 0;
}
