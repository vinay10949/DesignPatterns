use std::string;

struct Patient {
    name: String,
    registrationDone: bool,
    doctorCheckUpDone: bool,
    medicineDone: bool,
    paymentDone: bool,
}

trait Department<'a> {
    fn execute(&self, p: &mut Patient);
    fn set_next(&mut self, p: &'a mut dyn Department<'a>);
}

struct Reception<'a> {
    next: &'a dyn Department<'a> ,
}

impl<'a> Department<'a> for Reception<'a> {
    fn execute(&self, p: &mut Patient) {
        if p.registrationDone {
            println!("Patient registration already done");
            self.next.execute(p);
            return;
        }
        println!("Reception registering patient");
        p.registrationDone = true;
        self.next.execute(p);
    }
    fn set_next(&mut self, next: &'a mut dyn Department<'a>) {
        self.next=next
    }
}

struct Doctor<'a> {
    next: &'a dyn Department<'a> ,
}

impl<'a> Department<'a> for Doctor<'a> {
    fn execute(&self, p: &mut Patient) {
        if p.doctorCheckUpDone {
            println!("Doctor checkup already done ");
            self.next.execute(p);
            return;
        }
        println!("Doctor Checking patient");
        p.doctorCheckUpDone = true;
        self.next.execute(p);
    }
    fn set_next(&mut self, next: &'a mut dyn Department<'a>) {
        self.next=next
    }
}



struct Medical<'a> {
    next: &'a dyn Department<'a> ,
}

impl<'a> Department<'a> for Medical<'a> {
    fn execute(&self, p: &mut Patient) {
        if p.medicineDone {
            println!("Medicine already given to patient");
            self.next.execute(p);
            return;
        }
        println!("Medical giving medicine to patient");
        p.medicineDone = true;
        self.next.execute(p);
    }
    fn set_next(&mut self, next: &'a mut dyn Department<'a>) {
        self.next=next
    }
}


struct Cashier<'a> {
    next: Option<&'a dyn Department<'a>> ,
}

impl<'a> Department<'a> for Cashier<'a> {
    fn execute(&self, p: &mut Patient) {
        if p.paymentDone {
            println!("Payment Done");
            self.next.unwrap().execute(p);
            return;
        }
        println!("Cashier getting money from patient patient");
        p.paymentDone = true;
        if self.next.is_some(){
            self.next.unwrap().execute(p);
        }
    }
    fn set_next(&mut self, next: &'a mut dyn Department<'a>) {
        self.next=Some(next)
    }
}

fn main() {
   
    let cashier= &Cashier{next:None};

    //Set next for medical department
    let medical = &Medical{next:cashier};
   
    //Set next for doctor department
    let doctor = &Doctor{next:medical};

    //Set next for reception department
    let reception = &Reception{next:doctor};
  
    let  mut patient = &mut Patient{name: "abc".to_string(),registrationDone:false,medicineDone:false,doctorCheckUpDone:false,paymentDone:false};
    //Patient visiting
    reception.execute(patient);
}
