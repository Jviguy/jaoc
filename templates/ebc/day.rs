use {{project-name}}::ebc_main;

fn part1(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok(format!("Input has {} lines", input.lines().count()))
}

fn part2(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok("Part 2 not implemented".to_string())
}

fn part3(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok("Part 3 not implemented".to_string())
}

ebc_main!({{day}}, part1, part2, part3);
