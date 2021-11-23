// example 02
//
// introduces:
// - plane scrolling
// - formatted output
// - environment info
// - terminal capabilities

#include <notcurses/notcurses.h>

int main(void){
	struct notcurses_options nopts = {
		.flags =
			NCOPTION_NO_ALTERNATE_SCREEN |
			NCOPTION_SUPPRESS_BANNERS |
			NCOPTION_PRESERVE_CURSOR |
			NCOPTION_NO_CLEAR_BITMAPS |
			NCOPTION_DRAIN_INPUT
	};
	struct notcurses* nc = notcurses_core_init(&nopts, NULL);
	if(nc == NULL){
		return EXIT_FAILURE;
	}

	struct ncplane* stdn = notcurses_stdplane(nc);
	// set the standard plane to scroll
	ncplane_set_scrolling(stdn, true);

	// the standard plane size matches the terminal dimensions
	unsigned dimy, dimx;
	ncplane_dim_yx(stdn, &dimy, &dimx);

	// show the detected os version, terminal name & dimensions
	char* str_osversion = notcurses_osversion();
	char* str_terminal = notcurses_detected_terminal(nc);
	ncplane_printf_yx(stdn, -1, -1,
		"\nOperating System: %s\nTerminal: %s\n"
		"Dimensions: %i rows, %i cols\n\n",
		str_osversion, str_terminal, dimy, dimx
	);
	free(str_osversion);
	free(str_terminal);

	// show the terminal capabilities
	nccapabilities const* caps = notcurses_capabilities(nc);
	char str_caps[256];
	ncplane_printf_yx(stdn, -1, -1, "Capabilities:\n"
			"  utf8: %s\n  halfblocks: %s\n  quadrants: %s\n  sextants: %s\n"
			"  braille: %s\n  pixel: %s\n  24bit-color: %s\n"
			"  palette colors: %i\n",
			caps->utf8?"true":"false",
			caps->halfblocks?"true":"false",
			caps->quadrants?"true":"false",
			caps->sextants?"true":"false",
			caps->braille?"true":"false",
			notcurses_canpixel(nc)?"true":"false",
			caps->rgb?"true":"false",
			caps->colors
			);

	notcurses_render(nc);

	if (notcurses_stop(nc)) {
		return EXIT_FAILURE;
	}
	return EXIT_SUCCESS;
}
