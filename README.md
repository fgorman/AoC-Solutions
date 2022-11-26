# AoC-Solutions
Simple input retrieval and solution running of the AoC problems

### Setup
1. Clone the repo
2. Make sure that you have openssl development package installed (libssl-dev on Ubuntu and openssl-devel on Fedora)
3. Build the source from the AoC-Solutions directory using cargo build
4. Login to AoC
5. Find your session cookie in developer tools
6. Create an environment variable called SESSION_COOKIE

### Running
* Command line arguments
  * -d or --day: which solution you want to run
  * -y or --year: which year of AoC you want to run
  
### Implementing Solutions
You will find a file for each solution in aoc-lib/src/solutions/

Methods are already stubbed out and ready to be implemented
