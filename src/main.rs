use crate::io::ErrorKind::InvalidData;
use ansi_term::Colour;
use std::collections::HashMap;
use std::io;
use std::io::Error;

#[derive(Debug)]
struct Consumer {
    state: u32,
    req: u32,
}

#[derive(Debug)]
struct Provider {
    resources: u32,
    expenses: u32,
}

#[derive(Debug)]
struct P2C {
    resource_id: u32,
    consumer_id: u32,
    state: u32,
    cost: u32,
}

fn main() -> io::Result<()> {
    println!("введите колчиство потребителей");
    let consumers_count = get_params_from_keyboard();
    let mut consumers = HashMap::new();
    let mut providers = HashMap::new();
    for i in 1..consumers_count + 1 {
        consumers.insert(i, get_consumer(i));
    }
    println!("введите кол-во поставщиков");
    let provider_count: u32 = get_params_from_keyboard();
    for i in 1..provider_count + 1 {
        providers.insert(i, get_providers(i));
    }
    match chek_balance(&consumers, &providers) {
        Ok(val) => val,
        Err(er) => {
            println!("Задача не сбалансированна");
            return Err(er);
        }
    };
    let mut matrix = get_matrix(&consumers, &providers);
    draw_matrix(&mut consumers, &mut providers, &mut matrix);
    calculate(&mut consumers, &mut providers, &mut matrix);
    draw_matrix(&mut consumers, &mut providers, &mut matrix);
    println!(
        "Затраты {}",
        matrix
            .iter()
            .map(|e: &P2C| -> u32 { e.state * e.cost })
            .sum::<u32>()
    );
    Ok(())
}

fn draw_matrix(
    consumers: &mut HashMap<u32, Consumer>,
    providers: &mut HashMap<u32, Provider>,
    matrix: &mut Vec<P2C>,
) {
    println!("_____________________________________");
    println!("|Поставщик  |       Потребитель     |");
    println!("_____________________________________");
    let mut storage_ids = providers.keys().copied().collect::<Vec<_>>();
    storage_ids.sort();
    for storage_id in storage_ids.iter() {
        print!("|Потсащик №{}|", storage_id);
        for rec in matrix.iter() {
            if rec.resource_id == *storage_id {
                if rec.state > 0 {
                    print!(
                        "{}",
                        Colour::Green.paint(format!("|cт:{} дст:{} |", rec.cost, rec.state))
                    );
                } else {
                    print!("|cт:{} дст:{} |", rec.cost, rec.state);
                }
            }
        }
        print!("|Запас {}| \n", providers[storage_id].resources);
    }
    print!("|Потребность:    |");
    for (consumer_id, consumer) in consumers.iter() {
        print!("    {}    |", consumer.req);
    }
    print!("\n")
}

fn calculate(
    consumers: &mut HashMap<u32, Consumer>,
    providers: &mut HashMap<u32, Provider>,
    matrix: &mut Vec<P2C>,
) -> io::Result<()> {
    for record in matrix.iter_mut() {
        if providers[&record.resource_id].resources > consumers[&record.consumer_id].req {
            record.state = consumers
                .get_mut(&record.consumer_id)
                .expect("key error")
                .req;
            consumers
                .get_mut(&record.consumer_id)
                .expect("key error")
                .req = 0;
            providers
                .get_mut(&record.resource_id)
                .expect("key error")
                .resources = providers
                .get_mut(&record.resource_id)
                .expect("key error")
                .resources
                - record.state;
            providers
                .get_mut(&record.resource_id)
                .expect("key error")
                .expenses = providers
                .get_mut(&record.resource_id)
                .expect("key error")
                .expenses
                + record.state * record.cost
        } else {
            consumers
                .get_mut(&record.consumer_id)
                .expect("key error")
                .req = consumers
                .get_mut(&record.consumer_id)
                .expect("key error")
                .req
                - providers
                    .get_mut(&record.resource_id)
                    .expect("key error")
                    .resources;
            record.state = providers
                .get_mut(&record.resource_id)
                .expect("key error")
                .resources;
            providers
                .get_mut(&record.resource_id)
                .expect("key error")
                .resources = 0;
            providers
                .get_mut(&record.resource_id)
                .expect("key error")
                .expenses = providers
                .get_mut(&record.resource_id)
                .expect("key error")
                .expenses
                + record.state * record.cost;
        }
    }
    Ok(())
}

fn get_matrix(consumers: &HashMap<u32, Consumer>, providers: &HashMap<u32, Provider>) -> Vec<P2C> {
    let mut matrix = Vec::new();
    for (id_provider, provider) in providers.iter() {
        for (id_consumer, consumer) in consumers.iter() {
            matrix.push(P2C {
                resource_id: *id_provider,
                consumer_id: *id_consumer,
                state: 0,
                cost: {
                    println!(
                        "ведите затраты поставщика № {} на доставку к потребителю  № {}",
                        id_provider, id_consumer
                    );
                    get_params_from_keyboard()
                },
            })
        }
    }
    matrix.sort_by(|a, b| a.resource_id.cmp(&b.resource_id));
    matrix
}

fn chek_balance(
    consumers: &HashMap<u32, Consumer>,
    providers: &HashMap<u32, Provider>,
) -> io::Result<()> {
    if consumers
        .iter()
        .map(|(id, x)| -> &u32 { &x.req })
        .sum::<u32>()
        != providers
            .iter()
            .map(|(id, x)| -> &u32 { &x.resources })
            .sum::<u32>()
    {
        return Err(Error::new(InvalidData, "Задача не сбалансирована"));
    }
    Ok(())
}

fn get_consumer(seq: u32) -> Consumer {
    println!(" введите потребность для потребителя № {} 📦📦📦📦", seq);
    Consumer {
        state: 0,
        req: get_params_from_keyboard(),
    }
}

fn get_providers(seq: u32) -> Provider {
    println!("введите запасы поставщика № {} 💲💲💲", seq);
    Provider {
        resources: get_params_from_keyboard(),
        expenses: 0,
    }
}

fn get_params_from_keyboard() -> u32 {
    let data = loop {
        let mut res = String::new();
        io::stdin()
            .read_line(&mut res)
            .expect("field in read stdin");
        let res: u32 = match res.trim().parse() {
            Ok(val) => val,
            Err(_) => {
                println!("🤡🤡 введена не цифра {} 🤡🤡", res);
                continue;
            }
        };
        break res;
    };
    data
}
