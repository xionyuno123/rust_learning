pub mod max_subarray {
    use std::ops::{Add, AddAssign};

    struct Value<T> {
        l:usize,
        r:usize,
        val:T,
    }

    /// Find maxinum nonempty subarray whose sum is largest.
    /// 
    /// Let’s think about how we might solve the maximum-subarray problem using
    /// the divide-and-conquer technique. Suppose we want to find a maximum subarray of the subarray 
    /// A[low..high]. Divide-and-conquer suggests that we divide
    /// the subarray into two subarrays of as equal size as possible. That is, we find
    /// the midpoint, say mid, of the subarray, and consider the subarrays A[low..mid]
    /// and A[mid+1..high]
    pub fn max_subarray_with_divide_and_conquer<'a,T>(arr:&'a Vec<T>)->&'a [T]
    where 
    T:Add+AddAssign+Clone+Ord,
    {
        if arr.len() ==0 {
            return &[];
        }
        
        let res=find_max_subarray(arr, 0, arr.len()-1);

        &arr[res.l..=res.r]
    }

    fn find_max_subarray<T>(arr:&Vec<T>,l:usize,r:usize)->Value<T>
    where T:Add+AddAssign+Clone+Ord
    {
        if l==r {
            return Value { l: l, r: r, val: arr[l].clone() };
        }

        let mid =(l+r)/2;

        let left=find_max_subarray(arr, l, mid);
        let right=find_max_subarray(arr, mid+1, r);
        let cross=find_cross_max_subarray(arr,l,mid,r);
        
        let mut ret;
        if left.val > right.val {
            ret=left;
        }
        else{
            ret=right;
        }

        if ret.val < cross.val {
            ret=cross;
        }

        ret

    }


    fn find_cross_max_subarray<T>(arr:&Vec<T>,l:usize,mid:usize,r:usize)->Value<T>
    where T:Add+AddAssign+Clone+Ord
    {
        let mut sum =arr[mid].clone();
        let mut left_big=sum.clone();
        let mut ret=Value{
            l:mid,
            r:mid+1,
            val:left_big.clone()
        };

        for i in (l ..mid).rev() {
          sum+=arr[i].clone();   
          if sum > left_big {
              left_big=sum.clone();
              ret.l=i;
          }
        }

        sum=arr[mid+1].clone();
        let mut right_big=sum.clone();
        for i in mid+2..=r {
            sum+=arr[i].clone();
            if sum >right_big {
                right_big=sum.clone();
                ret.r=i;
            }
        }

        ret.val=left_big;
        ret.val+=right_big;

        ret
    }



    /// Find maxinum nonempty subarry whose sum if largest
    /// 
    /// Dp solution
    pub fn max_subarray_with_dp<'a,T>(arr:&'a Vec<T>)->&'a [T]
    where T:Ord+Clone+AddAssign,
    {
        if arr.len()==0{
            return &[];
        }

        let mut dp:Vec<(usize,T)>=Vec::new();
        dp.reserve(arr.len());
        let mut max:(usize,usize,T)=(0,0,arr[0].clone());

        for (i,val) in arr.iter().enumerate() {
            if dp.is_empty() {
                dp.push((i,val.clone()));
                continue;
            }

            let mut sum =dp[i-1].1.clone();
            sum += val.clone();
            if sum > *val {
                dp.push((dp[i-1].0,sum));
            }
            else{
                dp.push((i,val.clone()));
            }

            if dp[i].1 > max.2 {
                max.2=dp[i].1.clone();
                max.0=dp[i].0;
                max.1=i;
            }
        }

        &arr[max.0..=max.1]

        
    }
    #[test]
    fn test_max_subarray(){
        let arr=vec![13,-3,-25,20,-3,-16,-23, 18,20,-7,12 ,-5,-22,15,-4,7];
        let slice=max_subarray_with_divide_and_conquer(&arr);

        assert_eq!(slice,&[18, 20, -7, 12][..]);

        let arr:Vec<i32>=Vec::new();
        let slice=max_subarray_with_divide_and_conquer(&arr);

        assert_eq!(slice,&[][..]);
        assert_eq!(slice,max_subarray_with_dp(&arr));

        let arr=vec![1,2,3,4,5,6,7,8,9,10];
        let slice=max_subarray_with_divide_and_conquer(&arr);

        assert_eq!(slice,&[1,2,3,4,5,6,7,8,9,10][..]);
        assert_eq!(slice,max_subarray_with_dp(&arr));

        let arr=vec![-2,1,-3,4,-1,2,1,-5,4];
        let slice=max_subarray_with_divide_and_conquer(&arr);

        assert_eq!(slice,&[4,-1,2,1][..]);
        assert_eq!(slice,max_subarray_with_dp(&arr));

        let arr=vec![1];
        let slice=max_subarray_with_divide_and_conquer(&arr);

        assert_eq!(slice,&[1][..]);
        assert_eq!(slice,max_subarray_with_dp(&arr));

    }


}


pub mod compare_sort {
    //! 本模块提供了多种数组排序算法的实现，包括插入排序，堆排序，快速排序，合并排序等
    //! 所有排序算法都提供了三种API： <NAME>_sort,<NAME>_sort_desc,<NAME>_sort_by
    //! 
    //! 特点：
    //! - 由于内部使用了core::ptr::swap，因此要求数组元素之间不能存在内存重叠。
    //! - 输入类型为AsMut<\[T\]>
    //! - 比较过程中不会交换相等元素。
    use core::cmp::Ordering;

    /// insert sort by asc
    /// 
    /// 空间复杂度：O(1) 时间复杂度：$O(n^2)$
    pub fn insert_sort<K,T>(arr:&mut K)
    where 
    K:AsMut<[T]>,
    T:Ord
    {
        insert_sort_by(arr, |x,y| {
            T::cmp(x, y)
        })
    }

    /// insert sort by desc
    /// 
    /// 空间复杂度：O(1) 时间复杂度：$O(n^2)$
    pub fn insert_sort_desc<K,T>(arr:&mut K)
    where 
    K:AsMut<[T]>,
    T:Ord
    {
        insert_sort_by(arr, |x,y| {
            T::cmp(y, x)
        })
    }


    /// insert sort 
    pub fn insert_sort_by<K,T,F>(arr:&mut K, mut is_less:F)
    where
        K:AsMut<[T]>,
        T:Ord,
        F:FnMut(&T,&T)->Ordering,
    {
        let slice=arr.as_mut();

        for i in 1..slice.len() {
            let mut j=i;

            while j>=1 && is_less(&slice[j],&slice[j-1]) == Ordering::Less {
                unsafe {
                    core::ptr::swap(&mut slice[j] as *mut T, &mut slice[j-1] as *mut T)
                }
                j-=1;
            }
        }
    }



    /// heap sort
    /// 
    /// 空间复杂度：$O(1)$ 时间复杂度: $O(nlgn)$
    /// 
    /// 表示堆的数组A包括两个属性：A.len表示数组元素的个数，A.heap-size表示有多少个堆元素存储在数组中。
    /// 只有A\[1..=A.heap-size\]中存放的是堆的有效元素。树的根结点是A\[1\]，这样给定一个结点的下标i，可以很容易地计算
    /// 得到它的父结点，左孩子和右孩子。
    /// 
    /// - **父结点** floor(i/2)
    /// - **左孩子** 2i
    /// - **右孩子** 2i+1
    /// 
    /// 这就是完全二叉树的数组存储方式。
    /// 
    /// 二叉堆可以分为两种形式：最大堆和最小堆，在这两种堆中，结点的值都要满足堆的性质
    /// 
    /// - 最大堆中，除了根结点以外的所有结点i都要满足：A\[parent(i)\] >= A\[i\]
    /// - 最小堆中，除了根结点以外的所有结点i都要满足：A\[parent(i)\] <= A\[i\]
    pub fn heap_sort<K,T>(arr:&mut K)
    where 
        T:Ord,
        K:AsMut<[T]>
    {
        heap_sort_by(arr,|x,y| {
            T::cmp(x, y)
        })
    }

    pub fn heap_sort_desc<K,T>(arr:&mut K)
    where 
        T:Ord,
        K:AsMut<[T]>
    {
        heap_sort_by(arr,|x,y| {
            T::cmp(y, x)
        })
    }

    pub fn heap_sort_by<K,T,F>(arr:&mut K,mut is_less:F)
    where
        K:AsMut<[T]>,
        T:Ord,
        F:FnMut(&T,&T)->Ordering,
    {
        let slice=arr.as_mut();
        for i in 2..=slice.len() {
            let mut k=i;
            while k>1 && is_less(&slice[k/2-1],&slice[k-1]) ==Ordering::Less {
                unsafe{
                    core::ptr::swap(&mut slice[k/2-1] as *mut T, &mut slice[k-1] as *mut T)
                };
                k=k/2;
            } 
        }

        for i in (2..=slice.len()).rev() {
            unsafe{
                core::ptr::swap(&mut slice[0] as *mut T,&mut slice[i-1] as *mut T)
            };
            
            let mut k=1;

            loop {
                let l=k*2;
                let r=k*2+1;
                let mut largest=k;
                if l<i && is_less(&slice[l-1],&slice[largest-1])==Ordering::Greater {
                    largest=l;
                }

                if r<i && is_less(&slice[r-1],&slice[largest-1])==Ordering::Greater {
                    largest=r;
                }

                if largest==k {
                    break;
                }
                unsafe{
                    core::ptr::swap(&mut slice[k-1] as *mut T,&mut slice[largest-1]  as *mut T)
                };
                k=largest;
            }
        }
    }

    pub fn merge_sort<K:AsMut<[T]>,T:Ord+Clone>(arr:&mut K) {
        merge_sort_by(arr,|x,y| T::cmp(x, y))
    }

    pub fn merge_sort_desc<K:AsMut<[T]>,T:Ord+Clone>(arr:&mut K) {
        merge_sort_by(arr,|x,y| T::cmp(y, x))
    }

    /// 归并排序的非递归实现
    /// 
    /// 非递归实现和递归实现的主要区别在于，非递归实现是自底向上的，先把数组分割成多个至多由两个
    /// 元素组成的子数组，然后依次归并。
    pub fn merge_sort_by<F,K,T>(arr:&mut K,mut is_less:F)
    where 
        F:FnMut(&T,&T)->Ordering,
        K:AsMut<[T]>,
        T:Ord+Clone,
    {
        let slice=arr.as_mut();
        

        let mut i=1;
        while i<slice.len() {
            let mut l=0;
            let mut mid =l+i-1;
            let mut r=mid+i;

            while r<slice.len() {
                merge(slice,l,mid,r,&mut is_less);
                l=r+1;
                mid=l+i-1;
                r=mid+i;
            }


            if l<slice.len() && mid < slice.len() {
                merge(slice,l,mid,slice.len()-1,&mut is_less);
            }
            i+=i;
        }
    }

    fn merge<F,T>(arr:&mut [T],low:usize,mid:usize,high:usize,is_less:&mut F)
    where 
        F:FnMut(&T,&T)->Ordering,
        T:Ord+Clone, 
    {
        let mut merge=Vec::with_capacity(high-low+1);
        if low< high{
            let mut l=low;
            let mut r=mid+1;
            while l<=mid && r<=high {
                if is_less(&arr[l],&arr[r])== Ordering::Less {
                    merge.push(arr[l].clone());
                    l+=1;
                }
                else{
                    merge.push(arr[r].clone());
                    r+=1;
                }
            }

            while l<=mid {
                merge.push(arr[l].clone());
                l+=1;
            }

            while r<=high {
                merge.push(arr[r].clone());
                r+=1;
            }

            arr[low..=high].clone_from_slice(&merge[..]);

        }
    }

    /// With O(lgn) stack space
    pub fn fast_sort<K:AsMut<[T]>,T:Ord+Clone>(arr:&mut K){
        fast_sort_by(arr, |x,y| T::cmp(x, y))
    }

    pub fn fast_sort_desc<K:AsMut<[T]>,T:Ord+Clone>(arr:&mut K){
        fast_sort_by(arr, |x,y| T::cmp(y, x))
    }

    /// 利用栈实现的非递归非随机快速排序算法
    pub fn fast_sort_by<K,T,F>(arr:&mut K,mut is_less:F)
    where
        T:Ord+Clone,
        F:FnMut(&T,&T)->Ordering,
        K:AsMut<[T]>,
    {
        let slice=arr.as_mut();
        if slice.len() == 0 {
            return;
        }

        let mut st=Vec::with_capacity(1<<(std::mem::size_of::<usize>()*8-slice.len().leading_zeros() as usize));

        st.push((0,slice.len()-1));

        while !st.is_empty() {
            let (l,r)=st.pop().unwrap();
            let (mut i,mut j)=(l as isize -1,r as isize +1);
            
            if l>=r {
                continue;
            }
            let k=slice[(l+r) as usize>>1].clone();
            while i<j {
                
                loop {
                    i+=1;
                    if is_less(&slice[i as usize],&k)!=Ordering::Less {
                        break;
                    }
                }

                loop {
                    j-=1;
                    if is_less(&slice[j as usize],&k)!=Ordering::Greater{
                        break;
                    }
                }


                if i<j {
                    unsafe{
                        core::ptr::swap(&mut slice[i as usize] as *mut T, &mut slice[j as usize] as *mut T)
                    };
                }
            }

            st.push((l,j as usize));
            st.push((j as usize +1,r));

        }
    }





    #[cfg(test)]
    mod test{
        use super::*;
        lazy_static::lazy_static!(
            static ref INPUT:Vec<Vec<i32>>={
                let mut ret = Vec::new();
                for _i in 0..100{
                    let len= _i*5;
                    ret.push(Vec::new());
                    for _j in 0..len{
                        ret[_i].push(rand::random());
                    }
                }
    
                ret
            };
        );
    
    
        #[test]
        fn test_insert_sort(){
            for arr in INPUT.iter() {
                let mut arr1=arr.clone();
                let mut arr2=arr.clone();
                arr1.sort();
                insert_sort(&mut arr2);
                assert_eq!(arr1,arr2);
            }
        }
        #[test]
        fn test_heap_sort(){
            for arr in INPUT.iter() {
                let mut arr1=arr.clone();
                let mut arr2=arr.clone();
                arr1.sort();
                heap_sort(&mut arr2);
                assert_eq!(arr1,arr2);
            }
        }
    
        #[test]
        fn test_merge_sort(){
            for arr in INPUT.iter() {
                let mut arr1=arr.clone();
                let mut arr2=arr.clone();
                arr1.sort();
                merge_sort(&mut arr2);
                assert_eq!(arr1,arr2);
            }
        }

        #[test]
        fn test_fast_sort(){
            for arr in INPUT.iter() {
                let mut arr1=arr.clone();
                let mut arr2=arr.clone();
                arr1.sort();
                fast_sort(&mut arr2);
                assert_eq!(arr1,arr2);
            }
        }
    }
    

}