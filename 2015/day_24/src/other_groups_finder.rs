use crate::package_group::PackageGroup;

pub fn find_other_groups(mut package_grp:  PackageGroup, third_of_total_val: i32) -> Vec<PackageGroup> {
    let mut ret_list: Vec<PackageGroup> = Vec::new();
    let working_nrs = package_grp.remaining_nrs.clone();

    find_second_set(&working_nrs, third_of_total_val, &mut package_grp, &mut ret_list);
    determine_remaining_nrs(&mut ret_list, &package_grp.remaining_nrs);
    ret_list
}

fn find_second_set(remaining_numbers: &[i32], 
                    third_of_total_val: i32, 
                    working_grp: &mut PackageGroup, 
                    return_list: &mut Vec<PackageGroup>) {
    let mut total_val : i32 = 0;
    working_grp.second_grp.iter().for_each(| int| total_val += *int );

    //can't use match statements when comparing to runtime values
    if total_val > third_of_total_val {
        return;
    }
    if total_val == third_of_total_val {
        //return valid combo
        working_grp.calculate_quantum_entanglement();
        return_list.push(working_grp.clone());
        return
    }
    if total_val < third_of_total_val {
        for index in 0..remaining_numbers.len() {
            working_grp.second_grp.push(*remaining_numbers.get(index).unwrap());
            find_second_set(&remaining_numbers[index+1..], third_of_total_val, working_grp, return_list);
            _ = working_grp.second_grp.pop();
        }
    }
}

fn determine_remaining_nrs(pack_list: &mut Vec<PackageGroup>, original_nr_list: &[i32])
{
    pack_list.iter_mut().for_each(| package | {
        let iter = original_nr_list.iter().filter(| number | {
            !package.first_grp.contains(*number) && !package.second_grp.contains(*number)
        });

        for item in iter {
            package.third_grp.push(*item);
        }

        package.remaining_nrs.clear();
    })
}