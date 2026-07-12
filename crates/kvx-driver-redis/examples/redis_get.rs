client.execute(Put::new("hello", "world")).await?;

println!("{:?}", client.execute(Get::new("hello")).await?);

client.execute(Delete::new("hello")).await?;

println!("{:?}", client.execute(Get::new("hello")).await?);