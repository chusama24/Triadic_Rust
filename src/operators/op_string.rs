
use crate::data_types::t_string::TString;
use crate::data_types::triadic::Triadic;
use crate::data_types::triadic_type::Ttypes;
use crate::t_enum::Degree;

use super::triadic_op::TriadicStringOp;

impl TriadicStringOp for TString{
        // First Triadic AND operator (Z) declared as tand_1. Its rule is F > L > T
    
        fn tand_1<B: Ttypes + Clone>(&self, v2: &B) -> Triadic {
            let temp = self.get_degree();
            let t_val = temp.get_value();
    // Match is just like a switch statement.
            match t_val{
                Degree::T => {
                    // If we do not put semi-colon at the end of the last statement, it will be considered
                    // the returning value .i.e. "return v2.get_degree();" === "v2.get_degree()"
                    v2.get_degree()
                },
                Degree::L => {
                    self.get_degree()
                },
                Degree::F => {
                    self.get_degree()
                }
            }
        }
    
        // Second Triadic AND operator (Omega) declared as tand_2. Its rule is L > F > T.
    
        fn tand_2<B: Ttypes + Clone>(&self, v2: &B) -> Triadic {
            let temp = self.get_degree();
            let t_val = temp.get_value();
            match t_val{
                Degree::T => {
                    v2.get_degree()
                },
                Degree::L => {
                    self.get_degree()
                },
                Degree::F => {
                    let v2_deg = v2.get_degree().get_value(); 
                    if v2_deg == Degree::L{  
                        v2.get_degree()
                    }
                    else{ 
                        self.get_degree()
                    }
                }
            }
        }
    // Third Triadic AND operator (Psi) declared as tand_3. Its rule is F > T > L.
    
        fn tand_3<B: Ttypes + Clone>(&self, v2: &B) -> Triadic {
            let temp = self.get_degree();
            let t_val = temp.get_value();
            match t_val{
                Degree::T => {
                    let v2_deg = v2.get_degree().get_value(); 
                    if v2_deg == Degree::L{  
                        self.get_degree()
                    }
                    else{ 
                        v2.get_degree()
                    }
                },
                Degree::L => {
                    v2.get_degree()
                },
                Degree::F => {
                    self.get_degree()
                }
            }
        }
    // Singular Triadic AND operator (combining all three ANDs) declared as tand. 
    
    // It simply calls tand_1, tand_2, and tand_3 triadic operators defined above.
        fn tand<B: Ttypes + Clone>(&self, v2: &B) -> Triadic {
            
            // AND1
            let mut t1 = Self::default();
            t1.set_degree(self.tand_1(v2));
    
            // AND2
            let mut t2 = Self::default();
            t2.set_degree(self.tand_2(v2));
    
    
            // AND3
            let mut t3 = Self::default();
            t3.set_degree(self.tand_3(v2));
    
    
            // AND3 WITH AND1 & AND2
            let mut  t4 = Self::default();
            t4.set_degree(t1.tand_3(&t2));
    
    
            // AND3 WITH AND1 AND AND3
                let mut t5 = Self::default();
                t5.set_degree(t1.tand_3(&t3));
    
            // AND3 WITH AND2 AND AND3
            let mut t6 = Self::default();
            t6.set_degree(t2.tand_3(&t3));
    
            // AND3 WITH AND1 AND2 AND3
            let mut t7 = Self::default();
            t7.set_degree(t4.tand_3(&t5));
    
            let t8 = t6.tand_3(&t7);
            return t8;
        }
    // First Triadic OR operator (Omega) declared as tor_1. Its rule is T > L > F
    
        fn tor_1<B: Ttypes + Clone>(&self, v2: &B) -> Triadic {
            let temp = self.get_degree();
            let t_val = temp.get_value();
            match t_val{
                Degree::T => {
                    self.get_degree()
                },
                Degree::L => {
                    let v2_deg = v2.get_degree().get_value(); 
                    if v2_deg == Degree::F{  
                        self.get_degree()
                    }
                    else{ 
                        v2.get_degree()
                    }
                },
                Degree::F => {
                    v2.get_degree()            
                }
            }
        }
    // Second Triadic OR operator (Y) declared as tor_2. Its rule is L > T > F
    
        fn tor_2<B: Ttypes + Clone>(&self, v2: &B) -> Triadic {
            let temp = self.get_degree();
            let t_val = temp.get_value();
            match t_val{
                Degree::T => {
                    let v2_deg = v2.get_degree().get_value(); 
                    if v2_deg == Degree::F{  
                        self.get_degree()
                    }
                    else{ 
                        v2.get_degree()
                    }
                },
                Degree::L => {
                    self.get_degree()
                    
                },
                Degree::F => {
                    v2.get_degree()              
                }
            }
        }
    // Third Triadic OR operator (Phi) declared as tor_3. Its rule is T > F > L
    
        fn tor_3<B: Ttypes + Clone>(&self, v2: &B) -> Triadic {
            let temp = self.get_degree();
            let t_val = temp.get_value();
            match t_val{
                Degree::T => {
                    self.get_degree()              
                },
                Degree::L => {
                    v2.get_degree()
                },
                Degree::F => {
                    let v2_deg = v2.get_degree().get_value(); 
                    if v2_deg == Degree::L{  
                        self.get_degree()
                    }
                    else{ 
                        v2.get_degree()
                    }
                }
            }
        }
    // Singular Triadic OR operator (combining all three ORs) declared as tand. 
    
    // It simply calls tor_1, tor_2, and tor_3 triadic operators defined above.  
        fn tor<B: Ttypes + Clone + Clone>(&self, v2: &B) -> Triadic {
             // OR1
             let mut t1 = Self::default();
             t1.set_degree(self.tor_1(v2));
     
             // OR2
             let mut t2 = Self::default();
             t2.set_degree(self.tor_2(v2));
     
     
             // OR3
             let mut t3 = Self::default();
             t3.set_degree(self.tor_3(v2));
     
     
             // OR3 WITH OR1, OR2
             let mut  t4 = Self::default();
             t4.set_degree(t1.tor_3(&t2));
     
     
             // OR3 WITH OR1, OR3
                 let mut t5 = Self::default();
                 t5.set_degree(t1.tor_3(&t3));
     
             // OR3 WITH OR2, OR3
             let mut t6 = Self::default();
             t6.set_degree(t2.tor_3(&t3));
     
             // OR3 WITH OR1, OR2, OR3
             let mut t7 = Self::default();
             t7.set_degree(t4.tor_3(&t5));
     
             let  t8 = t6.tor_3(&t7);
             return t8;
        }
    // First Triadic NOT operator (strong-negation) declared as tnot_1. Its rule is T -> F; F -> T; L -> F.
    
        fn tnot_1(&self) -> Triadic {
            match self.get_degree().get_value(){
                Degree::T => {
                    Triadic::new(Degree::F)             
                },
                _ => {
                    Triadic::new(Degree::T)             
                }
            }
        }
    // Second Triadic NOT operator (partial-negation) declared as tnot_2. Its rule is T -> L; F -> L; L -> L.
    
        fn tnot_2(&self) -> Triadic {
            Triadic::new(Degree::L)             
        }
    // Third Triadic NOT operator (Forward Cyclic-negation) declared as tnot_2. Its rule is T -> L; L -> F; F -> T.
    
        fn tnot_3(&self) -> Triadic {
            match self.get_degree().get_value(){
                Degree::T => {
                    Triadic::new(Degree::L)              
                },
                Degree::L => {
                    Triadic::new(Degree::F)     
                },
                Degree::F => {
                    Triadic::new(Degree::T)
                }
            }
        }
    // Fourth Triadic NOT operator (Backward Cyclic-negation) declared as tnot_4. Its rule is T -> F; L -> T; F -> L.
    
        fn tnot_4(&self) -> Triadic {
            match self.get_degree().get_value(){
                Degree::T => {
                    Triadic::new(Degree::F)     
                },
                Degree::L => {
                    Triadic::new(Degree::T)     
                },
                Degree::F => {
                    Triadic::new(Degree::L)     
                }
            }
        }

    fn tplus_1(&self, v2: Self) -> Self {
        let mut obj = Self::default();
       let temp = self.tand_1(&v2);

        match temp.get_value(){
            Degree::T =>{
                obj.set_degree(Triadic::new(Degree::T));
                    obj.set_value(self.get_value() + &v2.get_value());
            },
            Degree::L =>{
                obj.set_degree(Triadic::new(Degree::L));
            },
            Degree::F =>{
                obj.set_degree(Triadic::new(Degree::F));
            }
        }
        return obj;
    }

    fn tplus_2(&self, v2: Self) -> Self {
        let mut obj = Self::default();
        let temp = self.tand_2(&v2);

        match temp.get_value(){
            Degree::T =>{
                obj.set_degree(Triadic::new(Degree::T));
                obj.set_value(self.get_value() + &v2.get_value());
            },
            Degree::L =>{
                obj.set_degree(Triadic::new(Degree::L));
            },
            Degree::F =>{
                obj.set_degree(Triadic::new(Degree::F));
            }
        }
        return obj;
    }

    fn tplus_3(&self, v2: Self) -> Self {
        let mut obj = Self::default();        
        let temp = self.tand_3(&v2);

        match temp.get_value(){
            Degree::T =>{
                obj.set_degree(Triadic::new(Degree::T));
                obj.set_value(self.get_value() + &v2.get_value());
            },
            Degree::L =>{
                obj.set_degree(Triadic::new(Degree::L));                    
            },
            Degree::F =>{
                obj.set_degree(Triadic::new(Degree::F));                    
            }
        }
        return obj;
    }

    fn tplus(&self, v2: Self) -> Self {
        let mut obj = Self::default();        
        let temp = self.tand(&v2);

        match temp.get_value(){
            Degree::T =>{
                obj.set_degree(Triadic::new(Degree::T));
                obj.set_value(self.get_value() + &v2.get_value());
            },
            Degree::L =>{
                obj.set_degree(Triadic::new(Degree::L));                    
            },
            Degree::F =>{
                obj.set_degree(Triadic::new(Degree::F));                    
            }
        }
        return obj;
    }
}

