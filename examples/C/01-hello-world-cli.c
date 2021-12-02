// example 01
//
// introduces:
// - notcurses options, CLI mode
// - error managing

#include <notcurses/notcurses.h>

int main(void){
	// initialize notcurses with custom options
	struct notcurses_options nopts = {
		.flags =
			NCOPTION_SUPPRESS_BANNERS // don't show version & performance info
			| NCOPTION_PRESERVE_CURSOR // preserve the terminal cursor location
			| NCOPTION_NO_ALTERNATE_SCREEN // don't use the alternate screen
			| NCOPTION_NO_CLEAR_BITMAPS // don't clear preexisting bitmaps
			| NCOPTION_DRAIN_INPUT // don't handle input
	};
	struct notcurses* nc = notcurses_core_init(&nopts, NULL);
	// check for initialization errors
	if(nc == NULL){
		return EXIT_FAILURE;
	}

	// get a reference to the standard plane
	struct ncplane* stdn = notcurses_stdplane(nc);

	// write to the standard plane at the current cursor coordinates
	ncplane_putstr_yx(stdn, -1, -1, "hello world");

	// render the standard pile
	notcurses_render(nc);

	// stop notcurses, checking for errors
	if (notcurses_stop(nc)) {
		return EXIT_FAILURE;
	}
	return EXIT_SUCCESS;
}
