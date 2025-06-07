#define NOB_IMPLEMENTATION

#include "nob.h"

#define BUILD_FOLDER "build/"
#define SRC_FOLDER   "src/"
#define AUTO_RUN     true
#define DEBUG        true

#if DEBUG
#define FLAGS "-g", "-O0", "-Wpedantic", "-DDEBUG"
#define FILE_NAME "debug"
#else
#define FLAGS "-O2", "-Wpedantic"
#define FILE_NAME "release"
#endif


int main(int argc, char **argv) {
    nob_minimal_log_level = NOB_WARNING;
    NOB_GO_REBUILD_URSELF(argc, argv);

    if (!nob_mkdir_if_not_exists(BUILD_FOLDER)) return 1;

    char* output = BUILD_FOLDER FILE_NAME;

    Nob_Cmd cmd = {0};
    nob_cc(&cmd);
    nob_cc_flags(&cmd);
    nob_cmd_append(&cmd, FLAGS);
    nob_cc_output(&cmd, output);

    nob_cc_inputs(&cmd, SRC_FOLDER "main.c");
    nob_cc_inputs(&cmd, SRC_FOLDER "log.c");
    nob_cc_inputs(&cmd, SRC_FOLDER "train_network.c");
    nob_cc_inputs(&cmd, SRC_FOLDER "train.c");
    nob_cc_inputs(&cmd, SRC_FOLDER "position.c");

    if (!nob_cmd_run_sync_and_reset(&cmd)) return 1;
    if (!AUTO_RUN) return 0;

    nob_cmd_append(&cmd, output);
    if (!nob_cmd_run_sync_and_reset(&cmd)) return 1;

    return 0;
}
