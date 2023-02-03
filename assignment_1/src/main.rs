  use std::io;
  use std::io::Write;

fn code_7() {
         println!("Enter name");
    let mut input1 = String::new();
       io::stdin().read_line(&mut input1).expect("Failed to read input ");
     let name:&str = input1.trim();
                 
    if  name== "Aigbona Juliet" {

         let info1 = "Department:Consulting
                Services : Analystics consulting services,Customer experience,
                Cyber security strategy risk compliance and resilience,Digital transformation,Risk consulting services
                Supply chain and operations,Commercial strategy
                Qualificiation:B.Sc.
                Code:7";

        let mut file1 = std::fs::File::create("Aigbona_Juilet.txt ").expect("create failed");
        file1.write_all("Aigbona Juliet".as_bytes()).expect("write failed");
        file1.write_all("This is the information for Aigbona Juilet".as_bytes()).expect("write failed");
        file1.write_all(info1.as_bytes()).expect("write failed");
        println!("Success");
    }

    else if name == "Ehis Ero"{
        let info2 = "Department:Strategy
                     Services:Strategy consulting,Corporate and growth strategy,Transaction strategy and execution,Restructuring and
                     turnaround strategy,Industry strategy,Digital business building,Commercial strategy
                     Qualificiation:M.Sc.
                     Code: 9";

           let mut file2 = std::fs::File::create("Ehis_Ero.txt ").expect("create failed");
        file2.write_all("Ehis Ero".as_bytes()).expect("write failed");
        file2.write_all("This is the information for Ehis Ero".as_bytes()).expect("write failed");
        file2.write_all(info2.as_bytes()).expect("write failed");
        println!("Success");


    }
      else if name== "Adamu Sagamu" {
          let info3 = "Department:Tax
                       Services:Tax planning,Tax function operations,Tax policy and controversy,Global trade
                       Tax accounting,Tax compliance,Transaction tax
                       Qualificiation:B.Sc.
                       Code:8";

             let mut file3 = std::fs::File::create("Adamu_Sagamu.txt ").expect("create failed");
        file3.write_all("Adamu Sagamu".as_bytes()).expect("write failed");
        file3.write_all("This is the information for Adamu Sagamu".as_bytes()).expect("write failed");
        file3.write_all(info3.as_bytes()).expect("write failed");
        println!("Success");

      }

        else if name== "Akpevwe Iloka" {
          let info4 = "Department:Assurance
                       Services:Audit services , Climate change and sustainability services
                       Financial accounting advisory services , Forensic and integrity services
                       Private client audit experience, Accounting Link , Assurance
                       Qualificiation:HND
                       Code:7";

             let mut file4 = std::fs::File::create("Akpevwe_Iloka.txt ").expect("create failed");
        file4.write_all("Akpevwe Iloka".as_bytes()).expect("write failed");
        file4.write_all("This is the information for Akpevwe  Iloka".as_bytes()).expect("write failed");
        file4.write_all(info4.as_bytes()).expect("write failed");
        println!("Success");
    }

        else if name== "Maria Akinsola" {
          let info5 = "Department:Transactions and corporate finance
                       Services:Corporate finance,Divestments and carve-outs,Sustainability and ESG services,M&A advisory
                       M&A integration,M&A technology and tools,M&A advanced analytics
                       Qualificiation:M.Sc.
                       Code:9";

             let mut file5 = std::fs::File::create("Maria_Akinsola.txt ").expect("create failed");
        file5.write_all("Maria Akinsola".as_bytes()).expect("write failed");
        file5.write_all("This is the information for Maria Akinsola".as_bytes()).expect("write failed");
        file5.write_all(info5.as_bytes()).expect("write failed");
        println!("Success");

         }

        else if name== "Gbenga Daniels" {
          let info6 = "Department:People and workforce
                       Services:Change management and experience,HR transformation,Integrated workforce mobility,Learning and 
                       development consulting, Recognition and reward advisory,Workforce analytics,People and workforce
                       Qualificiation:HND
                       Code:8";

             let mut file6 = std::fs::File::create("Gbenga_Daniels.txt ").expect("create failed");
        file6.write_all("Gbenga Daniels".as_bytes()).expect("write failed");
        file6.write_all("This is the information for Gbenga Daniels".as_bytes()).expect("write failed");
        file6.write_all(info6.as_bytes()).expect("write failed");
        println!("Success");
         }












           }
 
 fn main () {
    println!("Files for EY");
    code_7()
}