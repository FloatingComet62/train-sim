#define NOB_IMPLEMENTATION

#include "nob.h"

#define BUILD_FOLDER "build/"
#define SRC_FOLDER   "src/"

int main(int argc, char **argv) {
    bool auto_run = false;
    if (
        (argc > 1 && strcmp(argv[1], "quite") == 0) ||
        (argc > 2 && strcmp(argv[2], "quite") == 0)
    ) {
        nob_minimal_log_level = NOB_ERROR;
    }
    if (
        (argc > 1 && strcmp(argv[1], "run") == 0) ||
        (argc > 2 && strcmp(argv[2], "run") == 0)
     ) {
        auto_run = true;
    }
    NOB_GO_REBUILD_URSELF(argc, argv);

    if (!nob_mkdir_if_not_exists(BUILD_FOLDER)) return 1;

    char* output = BUILD_FOLDER"debug";

    Nob_Cmd cmd = {0};
    nob_cc(&cmd);
    nob_cc_flags(&cmd);
    nob_cc_output(&cmd, output);

    Nob_File_Paths src_files = {0};
    nob_read_entire_dir(SRC_FOLDER, &src_files);
    for (size_t i = 0; i < src_files.count; ++i) {
        if (src_files.items[i][0] == '.') continue; // Skip hidden files
        nob_log(NOB_INFO, "Adding source file: %s\n", src_files.items[i]);
        char full_path[1024];
        snprintf(full_path, sizeof(full_path), "%s%s", SRC_FOLDER, src_files.items[i]);

        nob_cc_inputs(&cmd, full_path);
    }

    if (!nob_cmd_run_sync_and_reset(&cmd)) return 1;
    if (!auto_run) return 0;

    nob_cmd_append(&cmd, output);
    if (!nob_cmd_run_sync_and_reset(&cmd)) return 1;

    return 0;
}
