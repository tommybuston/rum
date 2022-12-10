Group Members:

Tommy Buston
Brendan Rice

        ------------------------- Help / Collaboration -------------------------
We went to professor Daniels' office hours for help early on to discuss design plans for the UM.
We also attended Isaac's office hours for help completing the design doc as well as completing
the assignment.  We also talked with Cam McGuror about using a queue style vec for keeping track
of unmapped memory segments.

        ---------------------- What has been implemented? ----------------------
We successfully implemented a run_um module, a um_state module, a reg_math module, a segment_ops 
module, and a um_io module.  All of the functionality of these modules is detailed in Rust-Doc
style format.  These modules work together for what we believe to be a perfectly functional UM
becuase it is able to run many of the test binaries by Professor Daniels.

        ---------------------- Departures from Design --------------------------
There were two simple differences from our design plan to our actual implementation.  Number one 
was that we did not make "process_opcode" an actual module. Rather, it was contained in the 
implementation of the run_um module.  Another difference from our design plan was our approach
to tracking unmapped segments.  We were pretty unsure of how to accomplish this efficiently at
the design phase, but ended up choosing to implement it using a vec to act as queue for unmapped
segments (an idea suggested to us by Cameron McGuror as mentioned before).

        ---------------------- Our RUM Architecture ----------------------------
Our Rum begins by expecting a um or umz file from the command line at execution time.  We included
the load_um module provided in the rumdump lab.  If a file is not provided this module will resort
to taking in the um binary from standard input.  From here our run_um module takes over by 
initializing State and controlling the execution of the program.  The run_um module will unpack
the instruction words of the program, and perform the specified operation by calling to other modules.
This run time environment is driven by a loop that only stops when opcode 7 is read.  The other 
modules handle the operations specified in the assignment handout.  The reg_math module handles
operations that only deal with the values currently in registers.  The segment_ops module handles
operations that deal directed with the segmented memory.  The um_io module allows the RUM to 
do std in and std out operations.

        ---------------------- 50 Million Instructions -------------------------
Our machine currently took 7.48s to execute 50 million um binary instructions.  We know this 
because we added a counter to our loop that processes each instruction, and we set a breakpoint
once the counter reaches 50 million.  We used the gtime on the midmark.um test binary to actual
get an elapsed time of execution.

        ------------------ Time Spent Analyzing Assignment ---------------------
We spent around 2-3 hours analyzing the assignment including the time we spent discussing design
possibilities at Professor Daniels' office hours

        -------------------- Time Spent Preparing Design -----------------------
We spent around 4 hours preparing the design document including the time we spent at Isaac's OH.

        --------------------- Time Spent Implementing --------------------------
We spent around 15 hours preparing the entire RUM program altogether.

