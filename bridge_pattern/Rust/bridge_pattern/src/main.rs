trait Computer<'a> {
    fn print(&self);
    fn set_printer(&mut self, p: &'a (dyn Printer + 'a));
}

trait Printer {
    fn print_file(&self);
}

struct Mac<'a> {
    printer: &'a (dyn Printer + 'a),
}

struct Windows<'a> {
    printer: &'a (dyn Printer + 'a),
}

impl<'a> Computer<'a> for Mac<'a> {
    fn print(&self) {
        println!("Printing request for Mac");
        self.printer.print_file();
    }

    fn set_printer(&mut self, p: &'a dyn Printer) {
        self.printer = p;
    }
}

impl<'a> Computer<'a> for Windows<'a> {
    fn print(&self) {
        println!("Printing request for Windows");
       self.printer.print_file();
    }
    
    fn set_printer(&mut self, p: &'a dyn Printer) {
        self.printer = p;
    }
}

struct Epson {}

struct HP {}
impl Printer for Epson {
    fn print_file(&self) {
        println!("Printing by a EPSON Printer");
    }
}

impl Printer for HP {
    fn print_file(&self) {
        println!("Printing by a HP Printer");
    }
}

fn main() {
    let hpPrinter = &HP {};
    let epsonPrinter = &Epson {};

    let macComputer = &mut Mac { printer: hpPrinter };

    macComputer.print();
    println!("\n");

    macComputer.set_printer(epsonPrinter);
    macComputer.print();
    println!("\n");

    let winComputer = &mut Windows { printer: hpPrinter };

    winComputer.print();
    println!("\n");

    winComputer.set_printer(epsonPrinter);
    winComputer.print();
  println!("\n");

}
