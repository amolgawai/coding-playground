#ifndef CLI_OPTIONS_HPP
#define CLI_OPTIONS_HPP

#include <string>

class CLIOptions {
  public:
    CLIOptions(const std::string &app_descrption) { app_des = app_descrption;}
	virtual ~CLIOptions() {}
	bool scan_options(int argc, char** argv);

private:
	std::string app_des;
};

#endif
