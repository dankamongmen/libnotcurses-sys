// Sample program originally by Wyatt Sheffield (https://github.com/Wyatt915) at
//    https://wyatts.xyz/blog/writingasimpleno_2020-02-09
// Updated for 2025 by Jeff Stern (https://github.com/androclus)
//
// Draw 25 rows of 25 asterisks one cell at a time

#include <unistd.h> /* for sleep() */
#include <locale.h> /* for setlocale() */
#include <notcurses/notcurses.h>

int main(){

  // localization: some value is required by notcurses. (LC_ALL, "") is also fine.
  // See Dank's man page at
  // https://github.com/dankamongmen/notcurses/blob/master/doc/man/man3/notcurses.3.md
  setlocale(LC_ALL, ".UTF8");

  // declare the standard structure for setting global notcurses options
  notcurses_options ncopt;

  // and allocate the memory for it
  memset(&ncopt, 0, sizeof(ncopt));

  // initialize the main notcurses object and hold a handle to it
  struct notcurses* nc = notcurses_init(&ncopt, stdout);
  // check nc is good
  if(nc == NULL){
    // Note that the terminal may not be returned to standard I/O mode
    return EXIT_FAILURE;
  }

  // get a handle to the standard plane inside the notcurses object
  struct ncplane* stdplane = notcurses_stdplane(nc);

  // loop 25 rows and columns
  for (int i = 0; i < 25; i++){
    for (int j = 0; j < 25; j++){

      // put an asterisk at i,j on the plane..
      ncplane_putchar_yx(stdplane, i, j, '*');

      // ..and render the change to the screen (though this could be moved to
      // just before the sleep(2) for absolute fastest rendering)
      notcurses_render(nc);

      // sleep for 50 milliseconds after each character is drawn
      // (I don't know why Wyatt wanted to do this. Maybe just to
      // slow things down for effect. Or maybe to accommodate older
      // terminals?)
      sleep (0.05);
    }
  }

  // hold the picture on the screen for 2 seconds before we remove it
  sleep(2);

  // shut everything down, deallocate the memory, and return
  // the terminal to standard I/O mode. Check if successful.
  // Return non-zero error code if not.
  if (notcurses_stop(nc)) {
    // Note that the terminal may not be returned to standard I/O mode
    return EXIT_FAILURE;
  }

  // if we got this far, return success
  return EXIT_SUCCESS;
}
