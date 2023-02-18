use reqwest::{Body, Client, Response};
use pollster;
use tokio;


pub mod personal {
pub async fn validate(client: Client, first: i16, middle: i16, last: i16) -> bool {
    let mut params = HashMap::new();
    params.insert("area", first.to_string());
    params.insert("group", middle.to_string());
    params.insert("series", last.to_string());
    params.insert("_token", "69SOaHMJW4GGZO7SFumkBOTHRf0GOftOLEMgpp6I".to_string());

    let html = client.post("https://www.ssn-verify.com/")
        .form(&params)
        .send()
        .await.unwrap().text().await;
    println!("{:?}", &html);

    return false;
}

pub async fn check(first: u16) -> bool {
    let res = reqwest::get(format!("https://www.usatrace.com/ssn-search/Ethan-K-Kuhrts-{first}/"))
        .await.unwrap().text().await.unwrap();

    //let response = &res[res.find("</script><div class = \"alert alert-danger\">").unwrap_or(0)..res.find("</div><div class = \"well\">").unwrap_or(1000)];
    
    //let response = &res[res.find("There were no results for your search.").unwrap_or(res.len() - 10)..res.find("There were no results for your search.").unwrap_or(res.len() - 50s) + 50];
    
    
    if (res.find("There were no results for your search.").unwrap_or(0) == 0) {
        println!("something new!");
        return true;
    } else {
        println!("nah");
        return false;
    }
    
}


#[tokio::main]
async fn main() {
    /*
    let mut server: Server = Server::new(ServerInitOptions { 
        ip: "127.0.0.1".to_string(), 
        port: 7900, 
    });

    oobilydoop();
    let index = Route::new("/", || -> String {
        return "AA".to_string();
    });
    
    let mut router = server.router.lock().unwrap();
    router.register(index);

    drop(router);
    server.start();
    */

    //let client = Client::new();
    //println!("{:?}", pollster::block_on(validate(client, 602, 43, 2933)));

    let series = 2933;
    let possible: Vec<String> = Vec::new();

    let area_range1 = 545..573;
    let area_range2 = 600..650;
    let mut area_range1_vec: Vec<u16> = area_range1.collect();
    let area_range2_vec: Vec<u16> = area_range2.collect();
    for area in area_range2_vec {
        area_range1_vec.push(area);
    }

    let group_range_raw = 41..55;

    let group_range: Vec<i16> = group_range_raw.filter(|x| { x % 2 == 1 }).collect();

    let mut count = 0;
    
    for area in area_range1_vec {
        check(area).await;
        for group in &group_range {
            let ssn = format!("{}-{}-{}", area, group, series);
            println!("{}", ssn);
            count += 1;
        }
    }
    println!("{}", count);
}
}