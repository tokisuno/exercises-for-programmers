#include <stdio.h>
#include <stdbool.h>
#include <string.h>

int main(void)
{
    char input[50]; // must declare array size before string declaration

    for (;;) {
        printf("Input a string: ");
        if (fgets(input, sizeof(input), stdin) != NULL ) {
            input[strcspn(input, "\t\n")] = 0;
        }
        char *ptr = input;
        int counter;
        while (*ptr != '\0') {
            counter++;  // counts how many chars in string
            ptr++;      // moves the pointer to the next char
        }
        if (counter > 1) {
            printf("%s has %d characters\n", input, counter);
            break;
        } else {
            printf("Invalid input, try again!\n");
        }
    }

    return 0;
}
