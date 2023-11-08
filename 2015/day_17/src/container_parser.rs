use crate::ContainerType;

pub fn text_to_containers(text: &str) -> Vec<ContainerType>
{
    let mut container_list: Vec<ContainerType> = Vec::new();
    let trimmed_text = text.trim();
    for line in trimmed_text.lines()
    {
        let trimmed_line = line.trim();
        let capacity: u32 = trimmed_line.parse::<u32>().expect("trimmed line should only contain numbers");

        //let found_container_option = container_list.iter_mut().find(|container| container.capacity == capacity);
        // match found_container_option{
        //     Some(container_ref) => container_ref.increase_amount(),
        //     None => container_list.push(ContainerType::new(capacity)),
        // }
        container_list.push(ContainerType::new(capacity))
    }

    return container_list;
}