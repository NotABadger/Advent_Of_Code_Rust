
pub use crate::package_group::PackageGroup;


pub fn find_first_combo(numbers_list: &Vec<i32>, third_of_tot_val: i32) -> Vec<PackageGroup> {
    let mut ret_val: Vec<PackageGroup> = Vec::new();
    let mut working_nr_list: Vec<i32> = numbers_list.clone();
    working_nr_list.reverse(); //input file was already sorted from lowest to highest. But we will need the list to be sorted from highest to lowest
    let mut first_group: PackageGroup = PackageGroup::default();

    //recursive funtion.
    //Grab highest available number -> add to list
    //if list total > third_of_tot_val, remove last number & grab next
    //if list total == third_of_val, add package to list
    //if list total < third_of_val, recursive again
    //slices can be used to make sure we do not copy data all the time

    find_next_number(&working_nr_list, third_of_tot_val, &mut first_group, &mut ret_val);
    determine_remaining_nrs(&mut ret_val, &working_nr_list);
    ret_val
}

fn find_next_number(numbers_list: &[i32], third_of_tot_val: i32, working_grp: &mut PackageGroup, return_list: &mut Vec<PackageGroup>)
{
    let mut total_val : i32 = 0;
    working_grp.first_grp.iter().for_each(| int| total_val += *int );

    //can't use match statements when comparing to runtime values
    if total_val > third_of_tot_val {
        return;
    }
    if total_val == third_of_tot_val {
        return_list.push(working_grp.clone());
    }
    if total_val < third_of_tot_val {
        for index in 0..numbers_list.len() {
            working_grp.first_grp.push(*numbers_list.get(index).unwrap());
            find_next_number(&numbers_list[index+1..], third_of_tot_val, working_grp, return_list);
            _ = working_grp.first_grp.pop();
        }
    }
}


fn determine_remaining_nrs(pack_list: &mut Vec<PackageGroup>, original_nr_list: &[i32])
{
    pack_list.iter_mut().for_each(| package | {
        let iter = original_nr_list.iter().filter(| number | {
            !package.first_grp.contains(*number)
        });

        for item in iter {
            package.remaining_nrs.push(*item);
        }
    })
}