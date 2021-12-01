// example 00
// 
// introduces:
// - notcurses initialization & stopping
// - text outputting

#include <notcurses/notcurses.h>
#include <unistd.h> // necessary for `sleep()`

int main(void){
	// initialize notcurses with default options
	struct notcurses* nc = notcurses_core_init(NULL, NULL);

	// write to the standard plane at the top right corner
	ncplane_putstr_yx(notcurses_stdplane(nc), 0, 0, "hello world");

	// render the standard pile
	notcurses_render(nc);

	sleep(1);

	// stop notcurses
	notcurses_stop(nc);
}
