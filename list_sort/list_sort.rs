

fn main()
{
    let mut liste = vec![1,3,7,2,9,6,4,5];
    println!("{:?}", liste);
    println!("{:?}", bubble_sort(&mut liste));
    println!("{:?}", liste);
}


fn bubble_sort(l: &mut Vec<i32>)->Vec<i32>
{
    let mut tmp_value:i32;
    let mut tmp:usize;
    for ind in 0..=l.capacity()-1
    {
        tmp = ind;
        for j in ind..l.capacity()
        {
            tmp = if l[tmp] > l[j] {j} else {tmp};
        }
        if tmp != ind
        {
            tmp_value = l[ind];
            l[ind] = l[tmp];
            l[tmp] = tmp_value;
        }
    }
    return l.to_vec();
}
