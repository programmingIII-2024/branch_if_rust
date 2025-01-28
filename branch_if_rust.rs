fn main()
{

	// 分岐のテスト
	let mut n = 5;
	if n==1
	{
		println!("One");
		n+=1;
		println!("{}",n);
	}
	else if n >5
	{
		println!("Large");
	}
	else
	{
		println!("2 or 3 or 4 or 5");
	}

	// 変数宣言時にifの結果を代入
	let str = if n <5 {"Small"} else {"Large"};
	println!("{}",str);


//	C言語風のミスをした場合のテスト
/*
	if n=3
	{
		println!("C言語風のミスをしたら");
	}
*/	


}
