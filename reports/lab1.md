# 总结
sys_task_info主要实现：1.统计syscall被调用次数，2.给出当前任务距离第一次被调用的时间 3.显示当前任务状态（必然是running）

# 简答题

1. - bad_address: PageFault in application, kernel killed it. 用户程序不能访问0x0
   - bad_instructions: IllegalInstruction in application, kernel killed it. 用户程序无权使用S级指令
   - bad_register: IllegalInstruction in application, kernel killed it. 只有从用户trap到S级时，才会触发sstatus
   - RustSBI version 0.3.0-alpha.4, adapting to RISC-V SBI v1.0.0

2. 1. a0是保存了的任务上下文。两个应用场景：两个应用场景：（1）.系统从内核态切换至用户态  （2）.taskmanager切换至其他任务
   2. t0, t1 ,t2为临时寄存器，最后将32，33，2寄存器的值赋给sstatus（将 CPU 改为处在用户级），sepc（给出切换至用户态后首要执行的指令地址），sscratch（用于用户栈和内核栈的切换以及储存栈顶指针）
   3. 不知道
   4. sp指向用户栈，sscratch指向内核栈
   5. sret 该指令用于从中断或异常处理中返回调用者
   6. sp指向内核栈，sscratch指向用户栈
   7. 操作系统提供trap, 进入trap后由硬件完成

# 荣誉准则
1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

在小组群里问过`syscall_times: [u32; MAX_SYSCALL_NUM]`具体是什么

2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

简答题参考https://blog.csdn.net/surfaceyan/article/details/135030477

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。

