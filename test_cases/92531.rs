fn main(){
    let foos = vec![String::from("hello"), String::from("world")];
    let bars = vec!["world"];

    let mut baz = vec![];
    let mut qux = vec![];

    for foo in foos {
    for bar in &bars {
    if foo == *bar {
    baz.push(foo);
    continue;
    }
    }

    qux.push(foo);
    }

}