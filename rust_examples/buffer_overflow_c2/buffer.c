#include <stdio.h>
#include <string.h>
#include <stdlib.h>
int main(int argc, char *argv[])
{
    // Allocate a 6 byte buffer plus the NULL terminator
    char text_input[6]; // If this is more than 6 characters this will overflow

    printf("buffer content=%s\n", argv[1]);

    // Now copy the input to text_buffer, strcpy does not use bounds checking
    strcpy(text_input, argv[1]);
    printf("User Input:%s\n", text_input);

    return 0;
}
