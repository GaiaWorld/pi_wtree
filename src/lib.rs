//! 权重树
//! 用于存储带权重值的内容
//! 权重树将存储值组织成一颗二叉树，每个节点会记录节点内容、节点权重、节点权重和子节点权重的总和。
//! 同时，权重树中的二叉树是一个基于`节点权重`的大堆，从根到叶子节点，具有从大到小的顺序（但是左右节点的顺序不固定，这是堆的特性）
//! 如果从权重树的顶端一个一个的弹出内容，则总是先弹出权重值最大的节点。
//! 
//! 在实际应用过程中，权重树通常不从顶端依次弹出，而是更具外部指定的一个权重值。
//! 权重树的一个主要用途就是任务调度，通常在一个系统中，同一时间可能存在多个任务，不同任务可能具有不同的优先级。
//! 通常，我们希望先执行优先级更高的任务，但又不总是希望这样，在一些特殊情况下，
//! 如一个优先级低的任务加入了任务池，后面又源源不断地加入了优先级更高地任务，
//! 如果我们总是先执行优先级高的任务，那么在cpu密集的时间段，可能出现低优先级被饿死的情况。
//! 
//! 权重树可以和好的解决这个问题，并且相比传统方案，具有更高的性能。
//! 权重树的基本思想是，将任务根据权重组织为一个大堆，外部通过一个权重值来弹出一个任务，该权重值应该为一个`随机的`、`0~所有权重总和的`一个值.
//! 例如，我们有3个任务，它们的权重1、2、3， 那么，堆顶一定是最大的权重任务，剩下的两个任务（1、2）是堆顶任务（3）的子节点，它们的顺序不固定，我们假如2是3的左节点、1是3的右节点。在每个节点上记录了其自身权重。假如我们的节点数据结构为(自身权重),那么这三个节点分别是(3)、(2)、(1)。我们准备1+2+3个数字，用左闭右开的区间表示这些数字的话，即[1,7)。外部随机在这些数字中抽取一个数字，如果这个数字落在[1,4)中，这将弹出权重为3的任务；如果这个数字落在[4,6)中，则弹出权重为2的任务；如果这个数字落在[6,7)中，则弹出权重为1的任务；可以看到，权重为3的任务，弹出的概率为3/6 = 1/2;权重为2的任务，弹出的概率为2/6 = 1/3;权重为1的任务，弹出的概率为1/6 = 1/6;也正好符合我们的需求。具体算法为，假如随机到的数字是6，先找到根节点，发现6>3,跳过该节点，并且当前随机值减3，即value=6-3=3;继续遍历子节点，取到左节点，发现3>2, 跳过该节点, 并且当前随机值减2, 即value=3-2=1;继续遍历，取到右节点，发现1=1，则取除该节点，即：
//! 节点树为
//!    (3)
//! (2)  (1)
//! 
//! * 随机值为6，过程为：random=6; 6>3, random=6-3=3; 3>2, random=3-2=1; 1<=1, 弹出权重为1的任务
//! * 随机值为5，过程为：random=5; 5>3, random=5-3=2; 2<=2, 弹出权重为2的任务
//! * 随机值为4，过程为：random=4; 4>3, random=4-3=1; 1<=2, 弹出权重为2的任务
//! * 随机值为3，过程为：random=3; 3<=3, 弹出权重为3的任务
//! * 随机值为2，过程为：random=2; 2<=3, 弹出权重为3的任务
//! * 随机值为1，过程为：random=1; 1<=3, 弹出权重为3的任务
//! 
//! 上述例子中，我们的左子树权重为2，右子树权重为1，我们知道，在堆中，左右子树没有顺序要求，
//! 那么如果左子树为2，右子树为1，是什么情况呢?
//! 节点树为:
//!    (3)
//! (1) (2)
//! 
//! * 随机值为6，过程为：random=6; 6>3, random=6-3=3; 3>1, random=3-1=2; 2<=2, 弹出权重为2的任务
//! * 随机值为5，过程为：random=5; 5>3, random=5-3=2; 2>1, random=2-1=1; 1<=2, 弹出权重为2的任务
//! * 随机值为4，过程为：random=4; 4>3, random=4-3=1; 1<=1, 弹出权重为1的任务
//! * 随机值为3，过程为：random=3; 3<=3, 弹出权重为3的任务
//! * 随机值为2，过程为：random=2; 2<=3, 弹出权重为3的任务
//! * 随机值为1，过程为：random=1; 1<=3, 弹出权重为3的任务
//! 可以看出，实际上，元素的位置，只是影响了随机到哪些数字是，应该弹出哪个任务，
//! 但，权重为3的任务的弹出概率依然为1/2,权重为2的任务，弹出概率依然为1/3,权重为1的任务，弹出概率依然为1/6。依然满足我们的需求。
//! 事实上，即便我们的根节点是权重最要的任务（权重为1），也不影响它们各自弹出的概率。那么，为什么要组织为堆的结构？
//! 我们知道，权重越高的任务，弹出概率越大，并且，我们通过遍历的方式，寻找需要弹出的任务；
//! 如果能够将权重大的任务排在更前面，就能更快的找到对应任务。
//! 一种做法是，将任务组织为数组的结构，并对任务按照权重大小进行完全排序
//! 另一种做法是，将任务组织为堆，大致对其排序。
//! 
//! 我们选择了堆的数据结构，主要时因为：
//! 插入时性能更好。数组的结构，插入时，依次遍历每个节点，找到对应位置进行插入。堆的结构中，可以认为天然支持二分查找，找到对应位置并插入。
//! 那么，我们知道，如果使用数组全排序结构，弹出时，优先遍历权重大的任务，可以更快找到需要弹出的任务。如果使用堆，实际上可能先遍历到权重更小的任务，弹出性能是否不如数组呢？
//! 事实上，堆中的每个节点我们会额外存储一个值，即自身权重和字节点权重总和（后面我们同一称自身权重为self,总权重为all）
//! 通过对all和随机值得对比，我们可以选择是否要遍历该树的字节点，来达到二分查找的目的，比数组结构更快速！
//! 假如我们的节点数据结构为(自身权重、自身权重和字节点权重总和)，即（self，all），目前，我们有5个任务，签中分别是1、2、3、4、5、6，对应的树可能为：
//!              (6, 21)
//!     (4, 9)            (5, 1)
//! (2, 2) (3, 3)     (1, 1)
//! 在这种结构下，我们通过前面的描述可以知道，每个任务的数字分布情况为：
//! * 权重为6：[1, 7)
//! * 权重为4：[7, 11)
//! * 权重为2：[11, 13)
//! * 权重为3：[13, 16)
//! * 权重为5：[16, 21)
//! * 权重为1：[21, 22)
//! 
//! 有一种能快速定位弹出任务的方法。我们通过对比随机值和权重总和，来决定是否跳过一些子树。
//! 假如当前随机到的权重值为16：
//! random=16; 取到根节点（后面用层+左或右来表示，如权重为4的节点成为1左节点）, 16 > 6, random = 16-6 = 10;
//! random=10; 取到1左节点, 10 > 4, random = 10 - 4 = 6;    (注意此时，随机值减4之前为10，大于self+子节点总权重，即大于9，此时选择跳过遍历1左节点的子节点；即使继续遍历下去，你会发现，也无法找到想要的节点)
//! random=6; 直接跳过1左子树，取到1右节点， 6 > 5, randomOld = 6, random = 6 -5 = 1; randomOld <= 1右.all,接下来遍历子树
//! random=1; 取到2左节点， 1 <= 1, 弹出2左节点;
//! 

extern crate dyn_uint;
#[cfg(test)]
extern crate time;
#[cfg(test)]
extern crate rand;

pub mod wtree;
pub mod slab_wtree;
pub mod fast_wtree;