// The `derive` attribute automatically creates the implementation
// required to make this `enum` printable with `fmt::Debug`.
#[derive(Debug)]

enum Month_name {
    January,
    February,
    March,
    April,
    May,
    June,

    July,
    August,
    September,
    October,
    November,
    December
}

#[derive(Debug)]

enum Day_name {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday
}

#[derive(Debug)]

enum Country_name {
    Germany,
    India,
    Russia,
    France,
    Laos,
    Guinea,
    Ecuador
}

fn main() {
    let jan = Month_name :: January;
    let feb = Month_name :: February;
    let mar = Month_name :: March;
    let apr = Month_name :: April;
    let may = Month_name :: May;
    let jun = Month_name :: June;
    let jul = Month_name :: July;
    let aug = Month_name :: August;
    let sep = Month_name :: September;
    let oct = Month_name :: October;
    let nov = Month_name :: November;
    let dec = Month_name :: December;
    let tues = Day_name::Tuesday;
    let DE = Country_name::Germany;

    println!("{:?}",jan);
    println!("{:?}",feb);
    println!("{:?}",dec);
    println!("{:?}",tues);
    println!("The alternative name of DE  is {:?}",DE);

    //Calling the function in Scope of the main Function
    celsius_to_fahrenheit(30);
}

fn celsius_to_fahrenheit(c: i32) {

    let mut f = (c * 9/5) + 32;
    println!("{} degrees Celsius converts to {} degrees fahrenheit:", c,f);
}

