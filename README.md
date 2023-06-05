# A.R.M. SOFTWARE MANUAL
## Overview
A.R.M. (Automatic Repositioning Machine) is a robitic arm current in development by [@EricGarrelts](https://github.com/EricGarrelts) and [@asphergers](https://github.com/asphergers).

##

## Controls
The software currently operates in 3 basic modes, Normal Mode, Control Mode, and 
Buffer Mode
### Normal Mode
Normal Mode is used for performing basic operations outside the scope of actually controlling the robot <br />
* `ESC` : enter normal mode from anywhere
* `s` : Save the current angles of the robot to a text file
* `f` : flush the previous positions output
* `=` : increase the amount of previous points shown in the previous points output
* `\-` : decrease the amout of previous points shwon in the previous points output
* `]` :  increase the amount of points in the command output
* `[` : decrease the amount of points in the command output

### Control Moded
Control mode is used when controlling the robot and is where you'll spend most of your time
* `c` : enter control mode from normal mode
* <code>&uarr; &darr; &larr; &rarr;</code>  : directionally move the robot
* `\\` : read a position from the buffer and move there smoothly
* `ENTER` : read a position from the buffer and move there
* `=` : increase the amount the robot moves by every time you use a direction key
* `\-` : decrease the amount the robot moves by every time you use a direction key
* `]` : increase the maximum delay between steps when moving between points smoothly
* `[` : decrease the maximum delay between steps when moving between points smoothly
* `\'` : increase the minimum delay between steps when moving smoothly
* `;` : decrease the minimum delay between steps when moving smoothly
* `.` : increase the delay when moving normally
* `,` : decrease the delay when moving normally
* `m` : increase the step amount when moving the motors directly
* `n` : decrease the step amount when moving the motors directly
* `e` : move the beam posiiton clockwise, will not reflect in software
* `q` : move the beam counter clockwise, will not reflect in software
* `d` : move the column clockwise, will not reflect in software
* `a` : move the column counter clockwise, will not reflect in software

### Buffer Mode
Buffer mode is used to write to the buffer at the bottom of the screen
* `:` : enter buffer mode from normal mode
* `DEL` : clear the buffer
* `ENTER` : return to Normal mode
###
##

