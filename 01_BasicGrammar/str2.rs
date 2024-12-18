fn main() {
    // array = [类型; 长度] 这种语法对于i32、f64、bool等基础类型是OK的
    let a = [3u8; 5]; // a = [3, 3, 3, 3, 3]

    // 但是对于String这类非基础类型，需要用如下方式，因为基础类型数据是在栈内存，可以直接拷贝，
    // 而非基础类型的数据是在堆内存，需要深拷贝。
    let b: [String; 3] = std::array::from_fn(|_i| String::from("rust")); // b = ["rust","rust","rust"]

    let c = [9, 8, 7, 6, 5];
    // 通过下标直接访问
    let first = c[0]; // first = 9
    let second = c[1]; // second = 8

    // 访问不存在的元素，编译器会直接识别到并给出错误提示
    // let none_element = c[100];

    // arrays是一个二维数组，其中每一个元素都是一个数组，元素类型是[u8; 5]
		// arrays = [[3, 3, 3, 3, 3],[9, 8, 7, 6, 5]]
    let arrays: [[u8; 5]; 2] = [a, c];
}