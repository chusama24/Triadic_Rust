    use std::io;

    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Degree{
        T = 84,
        F = 70, 
        L = 76
    }

   pub trait ConvertToDegree {
        fn enum_convert(&self) -> Degree;
    }
    
    impl ConvertToDegree for char {
        fn enum_convert(&self) -> Degree {
            match self.to_ascii_lowercase() {
                't' => return Degree::T,
                'f' => return Degree::F,
                'l' => return Degree::L,
                _ => return Degree::L,
            };
        }
    }
    
    impl ConvertToDegree for i32 {
        fn enum_convert(&self) -> Degree {
            match *self {
                84  => return Degree::T,
                80 => return Degree::F,
                76 => return Degree::L,
                _ => return Degree::L,
            };
        }
    }

    
   pub fn t_scan(temp_input_degree: &mut Degree) {
        println!("Degree:");
        let mut input_double_degree = String::new();
        io::stdin().read_line(&mut input_double_degree).expect("Failed to read line");
        let input_double_degree: char = input_double_degree.trim().chars().next().unwrap_or('\0');
        *temp_input_degree = input_double_degree.enum_convert();
    }

    pub fn t_print(d: Degree) {
        match d {
            Degree::T => println!("True"),
            Degree::F => println!("False"),
            Degree::L => println!("Limit"),
        }
    }

