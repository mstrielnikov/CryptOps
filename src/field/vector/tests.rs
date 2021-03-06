mod tests{
	
	use crate::field::vector::native::*;
	use crate::operator::group::*;
	use crate::operator::logical::*;
	use crate::operator::convolution::*;
	use itertools::izip;
	use lazy_static::lazy_static;

	const TEST_CASES_NUM : usize = 5;

	lazy_static!{
		static ref TEST_X: [NativeVector; TEST_CASES_NUM] = [
			vec![1, 0, 1, 1], 			//1101
			vec![1, 1, 0, 1], 			//1011
			vec![1, 1, 0, 1, 1], 		//011011
			vec![1, 0, 1, 0, 1, 0, 1], 	//01010101
			vec![1, 1, 0, 1, 1, 0, 1]  	//01011011
		];
		
		static ref TEST_Y: [NativeVector; TEST_CASES_NUM] = [
			vec![1, 0, 0, 1], 			//1001
			vec![0, 1, 1, 0],			//0110
			vec![1, 1, 0, 1, 1], 		//011011
			vec![1, 1, 1, 1, 0, 0, 0], 	//00001111
			vec![1, 1, 0, 1, 1, 0, 1] 	//01011011
		];
	}

	#[test]
	fn test_add() {
		let mut test_vector: [NativeVector; TEST_CASES_NUM] = [
			vec![0; 5],
			vec![0; 5],
			vec![0; 6],
			vec![0; 8],
			vec![0; 8]
		];

		let expected: [NativeVector; TEST_CASES_NUM] = [
			vec![0, 1, 1, 0, 1], 		  	  //1101	+ 1001	  = 10110
			vec![1, 0, 0, 0, 1], 		  	  //1011	+ 110	  = 10001
			vec![0, 1, 1, 0, 1, 1], 	  	  //11011	+ 11011	  = 0110110
			vec![0, 0, 1, 0, 0, 1, 1, 0],	  //1010101 + 1111	  = 01100100
			vec![0, 1, 1, 0, 1, 1, 0, 1]  	  //1011011 + 1011011 = 10110110
		];
	
		for (test, x, y, ex) in izip!(&mut test_vector, TEST_X.iter(), TEST_Y.iter(), &expected)  {
			test.add(x, y);
			println!("#{:?} + {:?}", x, y);
			assert_eq!(test, ex);
		}
	}

	#[test]
	fn test_sub() {
		let mut test_vector: [NativeVector; TEST_CASES_NUM] = [
			vec![0; 4],
			vec![0; 4],
			vec![0; 5],
			vec![0; 7],
			vec![0; 7]
		];

		let expected: [NativeVector; TEST_CASES_NUM] = [
			vec![0, 0, 1, 0], 			//1101	  - 1001	= 00100
			vec![1, 0, 1, 0], 			//1011	  - 110		= 00101
			vec![0, 0, 0, 0, 0],		//11011	  - 11011	= 000000
			vec![0, 1, 1, 0, 0, 0, 1],	//1010101 - 1111    = 01000110
			vec![0, 0, 0, 0, 0, 0, 0]  	//1011011 - 1011011 = 00000000
		];
		
		for (test, x, y, ex) in izip!(&mut test_vector, TEST_X.iter(), TEST_Y.iter(), &expected)  {
			test.sub(x, y);
			println!("#{:?} - {:?}", x, y);
			assert_eq!(test, ex);
		}
	}

	#[test]
	fn test_eq() {
		let expected: [bool; TEST_CASES_NUM] = [
			false,  //1101 == 1001 = false
			false, 	//1011    == 110     = false
			true, 	//11011   == 11011   = true
			false,  //1010101 == 1111    = false
			true 	//1011011 == 1011011 = true
		];

		let mut got : bool;
		for (x, y, ex) in izip!(TEST_X.iter(), TEST_Y.iter(), &expected)  {
			got = x.equals(y);
			println!("#{:?} == {:?}", x, y);
			assert_eq!(&got, ex);
		}
	}

	#[test]
	fn test_gt() {
		let expected: [bool; TEST_CASES_NUM] = [
			true,   //1101 	  > 1001	= true
			true,   //1011 	  > 0110	= true
			false,  //11011   > 11011	= false
			true,   //1010101 > 1111	= true
			false   //1011011 > 1011011 = false
		];

		let mut got : bool;
		for (x, y, ex) in izip!(TEST_X.iter(), TEST_Y.iter(), expected.iter())  {
			got = x.greater(y);
			println!("#{:?} > {:?}", x, y);
			assert_eq!(&got, ex);
		}
	}
	

	#[test]
	fn test_gte() {
		let expected: [bool; TEST_CASES_NUM] = [
			true,  //1101    >= 1001    = true
			true,  //1011    >= 110     = true
			true,  //11011 	 >= 11011   = true
			true,  //1010101 >= 1111    = true
			true   //1011011 >= 1011011 = true
		];

		let mut got : bool;
		for (x, y, ex) in izip!(TEST_X.iter(), TEST_Y.iter(), expected.iter())  {
			got = x.greater_equals(y);
			println!("#{:?} >= {:?}", x, y);
			assert_eq!(&got, ex);
		}
	}

	#[test]
	fn test_ls() {
		let expected: [bool; TEST_CASES_NUM] = [
			false,  //1101    < 1001 	= false
			false,	//1011    < 110 	= false
			false,	//11011	  < 11011   = false
			false,	//1010101 < 1111    = false
			false	//1011011 < 1011011 = false
		];

		let mut got : bool;
		for (x, y, ex) in izip!(TEST_X.iter(), TEST_Y.iter(), expected.iter())  {
			got = x.lesser(y);
			println!("#{:?} < {:?}", x, y);
			assert_eq!(&got, ex);
		}
	}

	#[test]
	fn test_lse() {
		let expected: [bool; TEST_CASES_NUM] = [
			false, 	//1101    <= 1001 	 = false
			false,  //1011	  <= 110 	 = false
			true,  	//11011	  <= 11011	 = true
			false,	//1010101 <= 1111 	 = false
			true	//1011011 <= 1011011 = true
		];

		let mut got : bool;
		for (x, y, ex) in izip!(TEST_X.iter(), TEST_Y.iter(), expected.iter())  {
			got = x.lesser_equals(y);
			println!("#{:?} <= {:?}", x, y);
			assert_eq!(&got, ex);
		}
	}

	#[test]
	fn test_transform_x() {
		let mut test_vector: [NativeVector; TEST_CASES_NUM] = [
			vec![0; 8],		// 4 is tested vector size of half transform 
			vec![0; 8],		// 4 is tested vector size of half transform 
			vec![0; 10],	// 5 is tested vector size of half transform 
			vec![0; 12],	// 6 is tested vector size of half transform 
			vec![0; 14]		// 7 is tested vector size of half transform 
		];

		let expected: [NativeVector; TEST_CASES_NUM] = [
			vec![1, 0, 1, 2, 0, 0, 0, 0], 					//1101'=1012
			vec![0, 1, 2, 1, 0, 0, 0, 0], 					//1011'=0121
			vec![1, 2, 2, 2, 3, 0, 0, 0, 0, 0], 			//11011'=12223
			vec![1, 0, 2, 2, 1, 4, 3, 0, 0, 0, 0, 0, 0], 	//1011011'=1022143
			vec![0, 0, 0, 1, 1, 2, 2, 0, 0, 0, 0, 0, 0, 0]	//1010101'=0001122
		];

		for (test, x, y, ex) in izip!(&mut test_vector, TEST_X.iter(), TEST_Y.iter(), expected.iter())  {
			test.transform_x(x, y);
			println!("#{:?}' = {:?}", x, test);
			assert_eq!(ex, test);
		}
	}

	// #[test]
	// fn test_transform_y() {
	// 	let mut test_vector: [NativeVector; TEST_CASES_NUM] = [
	// 		vec![0; 8],		// 4 is tested vector size of half transform 
	// 		vec![0; 8],		// 4 is tested vector size of half transform 
	// 		vec![0; 10],	// 5 is tested vector size of half transform 
	// 		vec![0; 12],	// 6 is tested vector size of half transform 
	// 		vec![0; 14]		// 7 is tested vector size of half transform 
	// 	];

	// 	let expected: [NativeVector; TEST_CASES_NUM] = [
	// 		vec![0, 0, 0, 0, 0, 1, 1, 0], 				//1001'=0110
	// 		vec![0, 0, 0, 0, 1, 1, 0],					//0110'=110
	// 		vec![0, 0, 0, 0, 0, 4, 2, 1, 2, 1], 		//11011'=42121
	// 		vec![0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 1, 1], 	//0001111'=222211
	// 		vec![0, 0, 0, 0, 0, 0, 0, 2, 4, 2, 1, 2, 1]	//1011011'=242121
	// 	];

	// 	for (test, x, y, ex) in izip!(&mut test_vector, TEST_X.iter(), TEST_Y.iter(), expected.iter())  {
	// 		test.transform_y(x, y);
	// 		println!("#{:?}' = {:?}", y, test);
	// 		assert_eq!(ex, test);
	// 	}
	// }

	// 	#[test]
	// fn test_carry() {
	// 	let mut test_vector: [NativeVector; TEST_CASES_NUM] = [
	// 		vec![0; 8],
	// 		vec![0; 8],
	// 		vec![0; 10],
	// 		vec![0; 12],
	// 		vec![0; 14]
	// 	];
		
	// 	let expected: [NativeVector; TEST_CASES_NUM] = [
	// 		vec![1, 0, 1, 0, 1, 1, 1, 0], 					//=01110101
	// 		vec![0, 1, 0, 0, 0, 0, 1, 0], 					//=01000010
	// 		vec![1, 0, 0, 1, 1, 0, 1, 1, 0, 1], 			//=1011011001
	// 		vec![1, 1, 0, 1, 1, 1, 1, 1, 0, 0, 1, 0], 		//=1110000111001
	// 		vec![1, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1]  //=10000001011001
	// 	];
					
	// 	for (test, x, ex) in izip!(&mut test_vector, TEST_X.iter(), expected.iter())  {
	// 		test.carry();
	// 		println!("carry: #{:?} = {:?}", x, test);
	// 		assert_eq!(&test, &ex);
	// 	}
	// }

	// #[test]
	// fn test_mul() {
	// 	let mut test_vector: [NativeVector; TEST_CASES_NUM] = [
	// 		vec![0; 8],
	// 		vec![0; 8],
	// 		vec![0; 10],
	// 		vec![0; 12],
	// 		vec![0; 14]
	// 	];

	// 	let expected: [NativeVector; TEST_CASES_NUM] = [
	// 		vec![1, 0, 1, 0, 1, 1, 1, 0], 					//1101*1001=01110101
	// 		vec![0, 1, 0, 0, 0, 0, 1, 0], 					//1011*0110=01000010
	// 		vec![1, 0, 0, 1, 1, 0, 1, 1, 0, 1], 			//11011*11011=1011011001
	// 		vec![1, 1, 0, 1, 1, 1, 1, 1, 0, 0, 1, 0], 		//1010101*1111=1110000111001
	// 		vec![1, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1]  //1011011*1011011=10000001011001
	// 	];
			
	// 	for (test, x, y, ex) in izip!(&mut test_vector, TEST_X.iter(), TEST_Y.iter(), expected.iter())  {
	// 		test.mul(x, y);
	// 		println!("#{:?} * {:?}", x, y);
	// 		assert_eq!(test, ex);
	// 	}
	// }
	
/*
	#[test]
	fn test_sqr() {
		let expected: [bool; 4] = [
			NativeVector{vector: vec![1, 0, 1, 0, 1, 0, 0, 1]},
			NativeVector{vector: vec![1, 0, 1, 1, 0, 1, 1, 0, 0, 1, 0, 0]},
			NativeVector{vector: vec![1, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0]},
			NativeVector{vector: vec![1, 0, 1, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1]}
		];
		let mut got : NativeVector;
		for i in 0..4 {
			got = TEST_X[i].sqr();
			println!("{} : {:?} ^2  is {:?}", i, TEST_X[i].vector, got.vector);
			assert_eq!(got, expected[i]);
		}
	}

    #[test]
    fn test_exp() {
		let expected: [NativeVector; 4] = [
				NativeVector{vector: vec![1, 0, 0, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 1, 0, 1, 1, 1, 0, 0, 1]},
				NativeVector{vector: vec![1, 0, 0, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 1, 0, 1, 1, 1, 0, 0, 1]},
				NativeVector{vector: vec![1, 0, 0, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 1, 0, 1, 1, 1, 0, 0, 1]},
				NativeVector{vector: vec![1, 0, 0, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 1, 0, 1, 1, 1, 0, 0, 1]},
			];
		let test_mod: [NativeVector; 4] = [
			NativeVector{vector: vec![1, 0, 0, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 1, 0, 1, 1, 1, 0, 0, 1]},
			NativeVector{vector: vec![1, 0, 0, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 1, 0, 1, 1, 1, 0, 0, 1]},
			NativeVector{vector: vec![1, 0, 0, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 1, 0, 1, 1, 1, 0, 0, 1]},
			NativeVector{vector: vec![1, 0, 0, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 1, 0, 1, 1, 1, 0, 0, 1]},
		];
		let mut got : NativeVector;
		for i in 0..4 {
			got = TEST_X[i].exp(TEST_Y[i], test_mod);
			println!("{} : {:?} ^ {:?}  is {:?}", i, TEST_X[i].vector, TEST_Y[i].vector, got.vector);
			assert_eq!(got.vector, expected[i].vector);
		}         
    }
*/
/*
	#[test]
	fn test_rem() {
		const TEST_VECTOR_SIZE : usize = 8;
		let test_vector: [NativeVector; TEST_CASES_NUM] = [
			NativeVector{vector: vec![0; TEST_VECTOR_SIZE ]},
			NativeVector{vector: vec![0; TEST_VECTOR_SIZE ]},
			NativeVector{vector: vec![0; TEST_VECTOR_SIZE ]},
			NativeVector{vector: vec![0; TEST_VECTOR_SIZE ]}
		];
		let expected: [NativeVector; 4] = [
			NativeVector{vector: vec![1, 0, 0, 1]},
			NativeVector{vector: vec![0, 0, 0, 0, 0, 0]},
			NativeVector{vector: vec![0, 1, 0, 1, 1, 0, 1, 0]},
			NativeVector{vector: vec![0, 1, 1, 0, 1, 1]}
		];
		let mut got : NativeVector;
		for i in 0..4 {
			got = NativeVector{vector: TEST_X[i].vector.clone()};
			assert_eq!(got.rem(&TEST_Y[i]).vector, expected[i].vector);
			dbg!(i, &TEST_X[i].vector, &TEST_Y[i].vector);
		}
	}*/

}