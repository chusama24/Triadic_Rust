
use crate::data_types::t_f32::TF32;
use crate::data_types::triadic::Triadic;
use crate::data_types::triadic_type::Ttypes;
use crate::t_enum::Degree;
use crate::operators::triadic_op::TriadicOp;



impl TriadicOp for TF32 {

    fn tand_1<B: Ttypes>(self, v2: B) -> Triadic {
        let temp = self.get_degree();
        let t_val = temp.get_value();
    
        match t_val {
            Degree::T => {
                Triadic::new(v2.get_degree() as f32)
            },
            Degree::L => {
                Triadic::new(self.get_degree() as f32)
            },
            Degree::F => {
                Triadic::new(self.get_degree() as f32)
            }
        }
    }
    

    fn tand_2<B: Ttypes>(self, v2: B) -> Triadic {
        let temp = self.get_degree();
        let t_val = temp.get_value();
        match t_val {
            Degree::T => {
                Triadic::new(v2.get_degree() as f32)
            },
            Degree::L => {
                Triadic::new(self.get_degree() as f32)
            },
            Degree::F => {
                let v2_deg = v2.get_degree().get_value();
                if v2_deg == Degree::L {
                    Triadic::new(v2.get_degree() as f32)
                } else {
                    Triadic::new(self.get_degree() as f32)
                }
            }
        }
    }
    
    fn tand_3<B: Ttypes>(self, v2: B) -> Triadic {
        let temp = self.get_degree();
        let t_val = temp.get_value();
        match t_val {
            Degree::T => {
                let v2_deg = v2.get_degree().get_value();
                if v2_deg == Degree::L {
                    Triadic::new(self.get_degree() as f32)
                } else {
                    Triadic::new(v2.get_degree() as f32)
                }
            },
            Degree::L => {
                Triadic::new(v2.get_degree() as f32)
            },
            Degree::F => {
                Triadic::new(self.get_degree() as f32)
            }
        }
    }
    
    fn tand<B: Ttypes>(self, v2: B) -> Triadic {
    
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
        let mut t4 = Self::default();
        t4.set_degree(t1.tand_3(t2));
    
        // AND3 WITH AND1 AND AND3
        let mut t5 = Self::default();
        t5.set_degree(t1.tand_3(t3));
    
        // AND3 WITH AND2 AND AND3
        let mut t6 = Self::default();
        t6.set_degree(t2.tand_3(t3));
    
        // AND3 WITH AND1 AND2 AND3
        let mut t7 = Self::default();
        t7.set_degree(t4.tand_3(t5));
    
        let t8 = t6.tand_3(t7);
        return t8;
    }
    

    fn tor_1<B: Ttypes>(self, v2: B) -> Triadic {
    let temp = self.get_degree();
    let t_val = temp.get_value();
    match t_val {
        Degree::T => self.get_degree(),
        Degree::L => {
            let v2_deg = v2.get_degree().get_value();
            if v2_deg == Degree::F {
                self.get_degree()
            } else {
                v2.get_degree()
            }
        },
        Degree::F => v2.get_degree()
    }
}

fn tor_2<B: Ttypes>(self, v2: B) -> Triadic {
    let temp = self.get_degree();
    let t_val = temp.get_value();
    match t_val {
        Degree::T => {
            let v2_deg = v2.get_degree().get_value();
            if v2_deg == Degree::F {
                self.get_degree()
            } else {
                v2.get_degree()
            }
        },
        Degree::L => self.get_degree(),
        Degree::F => v2.get_degree()
    }
}

fn tor_3<B: Ttypes>(self, v2: B) -> Triadic {
    let temp = self.get_degree();
    let t_val = temp.get_value();
    match t_val {
        Degree::T => self.get_degree(),
        Degree::L => v2.get_degree(),
        Degree::F => {
            let v2_deg = v2.get_degree().get_value();
            if v2_deg == Degree::L {
                self.get_degree()
            } else {
                v2.get_degree()
            }
        }
    }
}

fn tor<B: Ttypes>(self, v2: B) -> Triadic {
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
    let mut t4 = Self::default();
    t4.set_degree(t1.tor_3(t2));

    // OR3 WITH OR1, OR3
    let mut t5 = Self::default();
    t5.set_degree(t1.tor_3(t3));

    // OR3 WITH OR2, OR3
    let mut t6 = Self::default();
    t6.set_degree(t2.tor_3(t3));

    // OR3 WITH OR1, OR2, OR3
    let mut t7 = Self::default();
    t7.set_degree(t4.tor_3(t5));

    let t8 = t6.tor_3(t7);
    return t8;
}

fn tnot_1(self) -> Triadic {
    match self.get_degree().get_value() {
        Degree::T => Triadic::new(Degree::F),
        _ => Triadic::new(Degree::T)
    }
}

    fn tnot_2(self) -> Triadic {
        Triadic::new(Degree::L)             
    }
// Third Triadic NOT operator (Forward Cyclic-negation) declared as tnot_2. Its rule is T -> L; L -> F; F -> T.

    fn tnot_3(self) -> Triadic {
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

    fn tnot_4(self) -> Triadic {
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

// First Triadic Less than (<) operator using Triadic AND1 (Z). 

    fn tlt_1(self, v2: Self) -> Self {
        
        let mut obj = Self::new(0,Triadic::default());
       let temp = self.tand_1(v2);

        match temp.get_value(){
            Degree::T =>{
                if self.get_value() < v2.get_value(){
                    obj.set_degree(Triadic::new(Degree::T));
                    obj.set_value(self.get_value());
                }
                else {
                    obj.set_degree(Triadic::new(Degree::F));
                    obj.set_value(v2.get_value());
                }
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
// Second Triadic Less than (<) operator using Triadic AND2 (omega). 

    fn tlt_2(self, v2: Self) -> Self {
        let mut obj = Self::new(0,Triadic::default());
        let temp = self.tand_2(v2);

        match temp.get_value(){
            Degree::T =>{
                if self.get_value() < v2.get_value(){
                    obj.set_degree(Triadic::new(Degree::T));
                    obj.set_value(self.get_value());
                }
                else {
                    obj.set_degree(Triadic::new(Degree::F));
                    obj.set_value(v2.get_value());
                }
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
// Third Triadic Less than (<) operator using Triadic AND1 (Psi). 

    fn tlt_3(self, v2: Self) -> Self {
        let mut obj = Self::new(0,Triadic::default());
        let temp = self.tand_3(v2);

        match temp.get_value(){
            Degree::T =>{
                if self.get_value() < v2.get_value(){
                    obj.set_degree(Triadic::new(Degree::T));
                    obj.set_value(self.get_value());
                }
                else {
                obj.set_degree(Triadic::new(Degree::F));
                    obj.set_value(v2.get_value());
                }
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
// Singular Triadic Less than (<) operator using Triadic AND1, AND2, and AND3. 

    fn tlt(self, v2: Self) -> Self {
        let mut obj = Self::new(0,Triadic::default());
        let temp = self.tand(v2);

        match temp.get_value(){
            Degree::T =>{
                if self.get_value() < v2.get_value(){
                    obj.set_degree(Triadic::new(Degree::T));
                    obj.set_value(self.get_value());
                }
                else {
                    obj.set_degree(Triadic::new(Degree::F));
                    obj.set_value(v2.get_value());
                }
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
// First Triadic Equavilence (==) operator using Triadic AND1 (Z). 

    fn teq_1(self, v2: Self) -> Self {
        let mut obj = Self::new(0,Triadic::default());
       let temp = self.tand_1(v2);

        match temp.get_value(){
            Degree::T =>{
                if self.get_value() == v2.get_value(){
                obj.set_degree(Triadic::new(Degree::T));
                    obj.set_value(self.get_value());
                }
                else {
                obj.set_degree(Triadic::new(Degree::F));
                    obj.set_value(v2.get_value());
                }
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


    fn teq_2(self, v2: Self) -> Self {
        let mut obj = Self::new(0,Triadic::default());
        let temp = self.tand_2(v2);

        match temp.get_value(){
            Degree::T =>{
                if self.get_value() == v2.get_value(){
                obj.set_degree(Triadic::new(Degree::T));
                    obj.set_value(self.get_value());
                }
                else {
                obj.set_degree(Triadic::new(Degree::F));
                    obj.set_value(v2.get_value());
                }
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


    fn teq_3(self, v2: Self) -> Self {
        let mut obj = Self::new(0,Triadic::default());        
        let temp = self.tand_3(v2);

        match temp.get_value(){
            Degree::T =>{
                if self.get_value() == v2.get_value(){
                obj.set_degree(Triadic::new(Degree::T));
                    obj.set_value(self.get_value());
                }
                else {
                obj.set_degree(Triadic::new(Degree::F));                    
                    obj.set_value(v2.get_value());
                }
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
// Singular Triadic Equavilence (==) operator using Triadic AND1, AND2, and AND3. 

    fn teq(self, v2: Self) -> Self {
        let mut obj = Self::new(0,Triadic::default());        
        let temp = self.tand(v2);

        match temp.get_value(){
            Degree::T =>{
                if self.get_value() == v2.get_value(){
                obj.set_degree(Triadic::new(Degree::T));                    
                    obj.set_value(self.get_value());
                }
                else {
                obj.set_degree(Triadic::new(Degree::F));                    

                    obj.set_value(v2.get_value());
                }
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
// First Triadic Greater than (>_1) operator using Triadic AND1 (Z). 

    fn tgt_1(self, v2: Self) -> Self {
        let mut obj = Self::new(0,Triadic::default());
       let temp = self.tand_1(v2);

        match temp.get_value(){
            Degree::T =>{
                if self.get_value() > v2.get_value(){
                obj.set_degree(Triadic::new(Degree::T));
                    obj.set_value(self.get_value());
                }
                else {
                obj.set_degree(Triadic::new(Degree::F));
                    obj.set_value(v2.get_value());
                }
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
// Second Triadic Equavilence (>_2) operator using Triadic AND2 (Omega). 

    fn tgt_2(self, v2: Self) -> Self {
        let mut obj = Self::new(0,Triadic::default());
        let temp = self.tand_2(v2);

        match temp.get_value(){
            Degree::T =>{
                if self.get_value() > v2.get_value(){
                obj.set_degree(Triadic::new(Degree::T));
                    obj.set_value(self.get_value());
                }
                else {
                obj.set_degree(Triadic::new(Degree::F));
                    obj.set_value(v2.get_value());
                }
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
// Third Triadic Equavilence (>_3) operator using Triadic AND3 (Psi). 

    fn tgt_3(self, v2: Self) -> Self {
        let mut obj = Self::new(0,Triadic::default());        
        let temp = self.tand_3(v2);

        match temp.get_value(){
            Degree::T =>{
                if self.get_value() > v2.get_value(){
                obj.set_degree(Triadic::new(Degree::T));
                    obj.set_value(self.get_value());
                }
                else {
                obj.set_degree(Triadic::new(Degree::F));                    
                    obj.set_value(v2.get_value());
                }
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
// Singular Triadic Equavilence (>) operator using Triadic AND1, AND2, and AND3. 

fn tgt(self, v2: Self) -> Self {
    let mut obj = Self::new(0,Triadic::default());        
        let temp = self.tand(v2);

        match temp.get_value(){
            Degree::T =>{
                if self.get_value() > v2.get_value(){
                obj.set_degree(Triadic::new(Degree::T));                    
                    obj.set_value(self.get_value());
                }
                else {
                obj.set_degree(Triadic::new(Degree::F));                    

                    obj.set_value(v2.get_value());
                }
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
    
    // First Triadic Greater than equal (>=_1) operator using Triadic AND1 (Z). 
    fn tgeq_1(self, v2: Self) -> Self {
        let mut obj = Self::new(0,Triadic::default());
        let temp = self.tand_1(v2);

        match temp.get_value(){
            Degree::T =>{
                if self.get_value() >= v2.get_value(){
                obj.set_degree(Triadic::new(Degree::T));
                    obj.set_value(self.get_value());
                }
                else {
                obj.set_degree(Triadic::new(Degree::F));
                    obj.set_value(v2.get_value());
                }
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
// Second Triadic Greater than equal (>=_2) operator using Triadic AND2 (Omega). 

    fn tgeq_2(self, v2: Self) -> Self {
        let mut obj = Self::new(0,Triadic::default());
        let temp = self.tand_2(v2);

        match temp.get_value(){
            Degree::T =>{
                if self.get_value() >= v2.get_value(){
                obj.set_degree(Triadic::new(Degree::T));
                    obj.set_value(self.get_value());
                }
                else {
                obj.set_degree(Triadic::new(Degree::F));
                    obj.set_value(v2.get_value());
                }
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
// Third Triadic Greater than equal (>=_3) operator using Triadic AND3 (Psi). 

    fn tgeq_3(self, v2: Self) -> Self {
        let mut obj = Self::new(0,Triadic::default());        
        let temp = self.tand_3(v2);

        match temp.get_value(){
            Degree::T =>{
                if self.get_value() >= v2.get_value(){
                obj.set_degree(Triadic::new(Degree::T));
                    obj.set_value(self.get_value());
                }
                else {
                obj.set_degree(Triadic::new(Degree::F));                    
                    obj.set_value(v2.get_value());
                }
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
// Singular Triadic Greater than equal (>=) operator using Triadic AND1, AND2, and AND3. 

    fn tgeq(self, v2: Self) -> Self {
        let mut obj = Self::new(0,Triadic::default());        
        let temp = self.tand(v2);

        match temp.get_value(){
            Degree::T =>{
                if self.get_value() >= v2.get_value(){
                obj.set_degree(Triadic::new(Degree::T));                    
                    obj.set_value(self.get_value());
                }
                else {
                obj.set_degree(Triadic::new(Degree::F));                    

                    obj.set_value(v2.get_value());
                }
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

    fn tleq_1(self, v2: Self) -> Self {
        let mut obj = Self::new(0,Triadic::default());
       let temp = self.tand_1(v2);

        match temp.get_value(){
            Degree::T =>{
                if self.get_value() <= v2.get_value(){
                obj.set_degree(Triadic::new(Degree::T));
                    obj.set_value(self.get_value());
                }
                else {
                obj.set_degree(Triadic::new(Degree::F));
                    obj.set_value(v2.get_value());
                }
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

    fn tleq_2(self, v2: Self) -> Self {
        let mut obj = Self::new(0,Triadic::default());
        let temp = self.tand_2(v2);

        match temp.get_value(){
            Degree::T =>{
                if self.get_value() <= v2.get_value(){
                obj.set_degree(Triadic::new(Degree::T));
                    obj.set_value(self.get_value());
                }
                else {
                obj.set_degree(Triadic::new(Degree::F));
                    obj.set_value(v2.get_value());
                }
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

    fn tleq_3(self, v2: Self) -> Self {
        let mut obj = Self::new(0,Triadic::default());        
        let temp = self.tand_3(v2);

        match temp.get_value(){
            Degree::T =>{
                if self.get_value() <= v2.get_value(){
                obj.set_degree(Triadic::new(Degree::T));
                    obj.set_value(self.get_value());
                }
                else {
                obj.set_degree(Triadic::new(Degree::F));                    
                    obj.set_value(v2.get_value());
                }
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

    fn tleq(self, v2: Self) -> Self {
        let mut obj = Self::new(0,Triadic::default());        
        let temp = self.tand(v2);

        match temp.get_value(){
            Degree::T =>{
                if self.get_value() <= v2.get_value(){
                obj.set_degree(Triadic::new(Degree::T));                    
                    obj.set_value(self.get_value());
                }
                else {
                obj.set_degree(Triadic::new(Degree::F));                    

                    obj.set_value(v2.get_value());
                }
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

    fn tneq_1(self, v2: Self) -> Self {
        let mut obj = Self::new(0,Triadic::default());
       let temp = self.tand_1(v2);

        match temp.get_value(){
            Degree::T =>{
                if self.get_value() != v2.get_value(){
                obj.set_degree(Triadic::new(Degree::T));
                    obj.set_value(self.get_value());
                }
                else {
                obj.set_degree(Triadic::new(Degree::F));
                    obj.set_value(v2.get_value());
                }
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

    fn tneq_2(self, v2: Self) -> Self {
        let mut obj = Self::new(0,Triadic::default());
        let temp = self.tand_2(v2);

        match temp.get_value(){
            Degree::T =>{
                if self.get_value() != v2.get_value(){
                obj.set_degree(Triadic::new(Degree::T));
                    obj.set_value(self.get_value());
                }
                else {
                obj.set_degree(Triadic::new(Degree::F));
                    obj.set_value(v2.get_value());
                }
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

    fn tneq_3(self, v2: Self) -> Self {
        let mut obj = Self::new(0,Triadic::default());        
        let temp = self.tand_3(v2);

        match temp.get_value(){
            Degree::T =>{
                if self.get_value() != v2.get_value(){
                obj.set_degree(Triadic::new(Degree::T));
                    obj.set_value(self.get_value());
                }
                else {
                obj.set_degree(Triadic::new(Degree::F));                    
                    obj.set_value(v2.get_value());
                }
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

    fn tneq(self, v2: Self) -> Self {
        let mut obj = Self::new(0,Triadic::default());        
        let temp = self.tand(v2);

        match temp.get_value(){
            Degree::T =>{
                if self.get_value() != v2.get_value(){
                obj.set_degree(Triadic::new(Degree::T));                    
                    obj.set_value(self.get_value());
                }
                else {
                obj.set_degree(Triadic::new(Degree::F));                    

                    obj.set_value(v2.get_value());
                }
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

    fn tplus_1(self, v2: Self) -> Self {
        let mut obj = Self::new(0,Triadic::default());
       let temp = self.tand_1(v2);

        match temp.get_value(){
            Degree::T =>{
                obj.set_degree(Triadic::new(Degree::T));
                    obj.set_value(self.get_value() + v2.get_value());
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

    fn tplus_2(self, v2: Self) -> Self {
        let mut obj = Self::new(0,Triadic::default());
        let temp = self.tand_2(v2);

        match temp.get_value(){
            Degree::T =>{
                obj.set_degree(Triadic::new(Degree::T));
                obj.set_value(self.get_value() + v2.get_value());
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

    fn tplus_3(self, v2: Self) -> Self {
        let mut obj = Self::new(0,Triadic::default());        
        let temp = self.tand_3(v2);

        match temp.get_value(){
            Degree::T =>{
                obj.set_degree(Triadic::new(Degree::T));
                obj.set_value(self.get_value() + v2.get_value());
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

    fn tplus(self, v2: Self) -> Self {
        let mut obj = Self::new(0,Triadic::default());        
        let temp = self.tand(v2);

        match temp.get_value(){
            Degree::T =>{
                obj.set_degree(Triadic::new(Degree::T));
                obj.set_value(self.get_value() + v2.get_value());
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

    fn tsub_1(self, v2: Self) -> Self {
        let mut obj = Self::new(0,Triadic::default());
       let temp = self.tand_1(v2);

        match temp.get_value(){
            Degree::T =>{
                obj.set_degree(Triadic::new(Degree::T));
                    obj.set_value(self.get_value() - v2.get_value());
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

    fn tsub_2(self, v2: Self) -> Self {
        let mut obj = Self::new(0,Triadic::default());
        let temp = self.tand_2(v2);

        match temp.get_value(){
            Degree::T =>{
                obj.set_degree(Triadic::new(Degree::T));
                obj.set_value(self.get_value() - v2.get_value());
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

    fn tsub_3(self, v2: Self) -> Self {
        let mut obj = Self::new(0,Triadic::default());        
        let temp = self.tand_3(v2);

        match temp.get_value(){
            Degree::T =>{
                obj.set_degree(Triadic::new(Degree::T));
                obj.set_value(self.get_value() - v2.get_value());
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

    fn tsub(self, v2: Self) -> Self {
        let mut obj = Self::new(0,Triadic::default());        
        let temp = self.tand(v2);

        match temp.get_value(){
            Degree::T =>{
                obj.set_degree(Triadic::new(Degree::T));
                obj.set_value(self.get_value() - v2.get_value());
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

    fn tmul_1(self, v2: Self) -> Self {
        let mut obj = Self::new(0,Triadic::default());
       let temp = self.tand_1(v2);

        match temp.get_value(){
            Degree::T =>{
                obj.set_degree(Triadic::new(Degree::T));
                    obj.set_value(self.get_value() * v2.get_value());
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

    fn tmul_2(self, v2: Self) -> Self {
        let mut obj = Self::new(0,Triadic::default());
        let temp = self.tand_2(v2);

        match temp.get_value(){
            Degree::T =>{
                obj.set_degree(Triadic::new(Degree::T));
                obj.set_value(self.get_value() * v2.get_value());
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

    fn tmul_3(self, v2: Self) -> Self {
        let mut obj = Self::new(0,Triadic::default());        
        let temp = self.tand_3(v2);

        match temp.get_value(){
            Degree::T =>{
                obj.set_degree(Triadic::new(Degree::T));
                obj.set_value(self.get_value() * v2.get_value());
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

    fn tmul(self, v2: Self) -> Self {
        let mut obj = Self::new(0,Triadic::default());        
        let temp = self.tand(v2);

        match temp.get_value(){
            Degree::T =>{
                obj.set_degree(Triadic::new(Degree::T));
                obj.set_value(self.get_value() * v2.get_value());
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

    fn tdiv_1(self, v2: Self) -> Self {
        let mut obj = Self::new(0,Triadic::default());
       let temp = self.tand_1(v2);

        match temp.get_value(){
            Degree::T =>{
                obj.set_degree(Triadic::new(Degree::T));
                    obj.set_value(self.get_value() / v2.get_value());
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

    fn tdiv_2(self, v2: Self) -> Self {
        let mut obj = Self::new(0,Triadic::default());
        let temp = self.tand_2(v2);

        match temp.get_value(){
            Degree::T =>{
                obj.set_degree(Triadic::new(Degree::T));
                obj.set_value(self.get_value() / v2.get_value());
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

    fn tdiv_3(self, v2: Self) -> Self {
        let mut obj = Self::new(0,Triadic::default());        
        let temp = self.tand_3(v2);

        match temp.get_value(){
            Degree::T =>{
                obj.set_degree(Triadic::new(Degree::T));
                obj.set_value(self.get_value() / v2.get_value());
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

    fn tdiv(self, v2: Self) -> Self {
        let mut obj = Self::new(0,Triadic::default());        
        let temp = self.tand(v2);

        match temp.get_value(){
            Degree::T =>{
                obj.set_degree(Triadic::new(Degree::T));
                obj.set_value(self.get_value() / v2.get_value());
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

    fn tmod_1(self, v2: Self) -> Self {
        let mut obj = Self::new(0,Triadic::default());
       let temp = self.tand_1(v2);

        match temp.get_value(){
            Degree::T =>{
                obj.set_degree(Triadic::new(Degree::T));
                    obj.set_value(self.get_value() % v2.get_value());
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

    fn tmod_2(self, v2: Self) -> Self {
        let mut obj = Self::new(0,Triadic::default());
        let temp = self.tand_2(v2);

        match temp.get_value(){
            Degree::T =>{
                obj.set_degree(Triadic::new(Degree::T));
                obj.set_value(self.get_value() % v2.get_value());
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

    fn tmod_3(self, v2: Self) -> Self {
        let mut obj = Self::new(0,Triadic::default());        
        let temp = self.tand_3(v2);

        match temp.get_value(){
            Degree::T =>{
                obj.set_degree(Triadic::new(Degree::T));
                obj.set_value(self.get_value() % v2.get_value());
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

    fn tmod(self, v2: Self) -> Self {
        let mut obj = Self::new(0,Triadic::default());        
        let temp = self.tand(v2);

        match temp.get_value(){
            Degree::T =>{
                obj.set_degree(Triadic::new(Degree::T));
                obj.set_value(self.get_value() % v2.get_value());
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