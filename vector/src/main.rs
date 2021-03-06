fn main() {
    //1
    let mut v: Vec<i32> = Vec::new();
    // v.push(1);

    //2
    let v = vec![1,2,3];
    //3
    {
        let v1 = vec![1,2,3];
        println!("v is {:?}",v1);
    }
    println!("v is {:?}",v);
    //4
    println!("v.1 is {:?}",v[0]);
    println!("v again is {:?}",v);


    match v.get(2) {
        Some(value) => println!("two is {:?}",value),
        _ => println!("None")
    }

    //5
    let mut v2: Vec<i32> = Vec::new();
    v2.push(1);
    v2.push(2);
    v2.push(3);

    // 6
    // 不可变
    let mut n = 1;
    for i in &v2 {
        println!("i{} = {}",n,i);
        n+=1
    }
    println!("n is {}",n);
    // 可变

    for i in &mut v2 {
        *i +=1;
        println!("i{} = {}",n,i);
    }

    {
        #[derive(Debug)]
        enum Context{
            Text(String),
            Float(f32),
            Int(i32),
        }

        let c = vec![Context::Text(String::from("String")),
                     Context::Int(-1),
                     Context::Float(0.001)
        ];
        println!("{:#?}",c)
    }


    {
        let mut v = vec![1,2,3,4,5];
        let mut first = &mut v[0];
        // v.push(6);
        println!("first = {}",first)
    }

    let s1 = String::from("this is s1");
    println!("{}",s1);
    let s1 = "this is s1 again".to_string();
    println!("{}",s1);

    for s in s1.chars() {
        println!("{}",s)
    };
    for s in s1.bytes() {
        println!("{}",s)
    };

    let s2 = String::from("你好啊");
    for c in s2.chars() {
        println!("{}",c)
    };

    for cb in s2.bytes() {
        println!("{}",cb)
    };

}
