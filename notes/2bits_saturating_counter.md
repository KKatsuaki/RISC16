# Branch Prediction



## State Machine for 2 bit Saturating Counter

the below table shows State Transition of 2 bit saturating counter

|       state        |        1         |         0          |
| :----------------: | :--------------: | :----------------: |
|   STRONGLY_TAKEN   |  STRONGLY_TAKEN  |    WEAKLY_TAKEN    |
|    WEAKLY_TAKEN    |  STRONGLY_TAKEN  |  WEAKLY_NOT_TAKEN  |
|  WEAKLY_NOT_TAKEN  |   WEAKLY_TAKEN   | STRONGLY_NOT_TAKEN |
| STRONGLY_NOT_TAKEN | WEAKLY_NOT_TAKEN | STRONGLY_NOT_TAKEN |

cite from P. 30 (18-447 Computer Architecture Lecture 11: Branch Prediction Prof. Onur Mutlu Carnegie Mellon University Spring 2013, 2/11, 2/11/2013)

## BPB 

**BPB** stands for *branch prediction buffer*. Its format would be below.