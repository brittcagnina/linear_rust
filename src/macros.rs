#[macro_export]
macro_rules! v {
    ( $( $x:expr ), * ) => {
	{
  	    let mut temp_vec = Vec::new();
	    $(
	        temp_vec.push($x);
	    )*
	    linrust::Vector::new(temp_vec)
	}
    };
}

#[macro_export]
macro_rules! row_matrix {
    ( $( $x: expr ), * ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            linrust::RowMatrix::new(temp_vec)
        }
    };
}

#[macro_export]
macro_rules! column_matrix {
    ( $( $x: expr ), * ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            ColumnMatrix::new(temp_vec)
        }
    };
}

#[macro_export]
macro_rules! add_overload {
    ($obj1: ident, $obj2: ident, $retType: ident, $s: ident, $o: ident, $logic: block) => { 
        impl Add<$obj1> for $obj2 { 
           type Output = $retType;
           fn add($s, $o: $obj1) -> $retType $logic
        }
    }; 
}

#[macro_export]
macro_rules! mul_overload {
    ($obj1: ident, $obj2: ident, $retType: ident, $s: ident, $o: ident, $logic: block) => { 
        impl Mul<$obj1> for $obj2 { 
           type Output = $retType;
           fn mul($s, $o: $obj1) -> $retType $logic
        }
    }; 
}
