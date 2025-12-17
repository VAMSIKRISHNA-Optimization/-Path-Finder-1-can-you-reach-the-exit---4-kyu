fn puzzle_str2vec(pzl: &str) -> (Vec<Vec<u8>>, u8)
{
    let mut pzl_vec:Vec<Vec<u8>> = Vec::new();
    
    let mut sub_vec: Vec<u8> = Vec::new();
    for char_v in pzl.chars()
    {
        if char_v == '.' { sub_vec.push(0);}
        if char_v == 'W' { sub_vec.push(1);}
        if char_v == '\n'{ pzl_vec.push(sub_vec.clone());  sub_vec = Vec::new();}
    }
    pzl_vec.push(sub_vec.clone());
    return (pzl_vec.clone(),  pzl_vec.len() as u8);
}


fn generate_coords_loops(n: u8) -> Vec<(u8, u8)> 
{
    let mut coords = Vec::new();

    // Iterate over rows (y-axis or first element of the tuple)
    for i in 0..n 
    {
        // Iterate over columns (x-axis or second element of the tuple)
        for j in 0..n 
        {
            coords.push((i, j));
        }
    }

    coords
}


fn check_left(coord:(u8,u8), len:u8)-> Vec<(u8, u8)>
{
    let mut new_pos : Vec<(u8,u8)> = Vec::new();
    
    if coord.1 != 0
    {
        new_pos.push((coord.0,coord.1-1))
    }
    new_pos
}

fn check_right(coord:(u8,u8), len:u8)-> Vec<(u8, u8)>
{
    let mut new_pos : Vec<(u8,u8)> = Vec::new();
    
    if coord.1 != len-1
    {
        new_pos.push((coord.0,coord.1+1))
    }
    new_pos
}

fn check_up(coord:(u8,u8), len:u8)-> Vec<(u8, u8)>
{
    let mut new_pos : Vec<(u8,u8)> = Vec::new();
    
    if coord.0 != 0
    {
        new_pos.push((coord.0-1,coord.1))
    }
    new_pos
}

fn check_down(coord:(u8,u8), len:u8)-> Vec<(u8, u8)>
{
    let mut new_pos : Vec<(u8,u8)> = Vec::new();
    
    if coord.0 != len-1
    {
        new_pos.push((coord.0+1,coord.1))
    }
    new_pos
}


fn path_finder(len:u8, puzz_vec:Vec<Vec<u8>>, poss_sol: Vec<(u8,u8)> )-> bool
{
    //poss_sol.push(start_coord.clone());

        for coord in poss_sol
        {
            if puzz_vec[coord.0 as usize][coord.1 as usize] != 1
            {
                let left_val = check_left(coord, len);
                let rig_val  = check_right(coord, len);
                let up_val   = check_up(coord, len);
                let down_val = check_down(coord, len);
                
                
                if !left_val.is_empty() 
                { 
                    if left_val[0] == (len-1, len-1) { return true; } 
                    //else {poss_sol.push(left_val[0]); } 
                }
                
                if !rig_val.is_empty() 
                { 
                    if rig_val[0] == (len-1, len-1) { return true; } 
                    //lse {poss_sol.push(rig_val[0]); } 
                }
                
                if !up_val.is_empty() 
                { 
                    if up_val[0] == (len-1, len-1) { return true; } 
                    //else {poss_sol.push(up_val[0]); } 
                }
                
                if !down_val.is_empty() 
                { 
                    if down_val[0] == (len-1, len-1) { return true; } 
                    //else {poss_sol.push(down_val[0]); } 
                }
            }
            

        }
        false
    
}




fn main ()
{
    let puzzle = format!("\
            .W.\n\
            .W.\n\
            W..\
            ");
            
    // println!("{:?}", puzzle);
    // println!("{:?}", puzzle_str2vec(&puzzle));
   
    // println!("{}", num_paths((5,5),6));
    // println!("{}", num_paths((0,0),6));
    
    // println!("{:?}", check_left((2,5),6));
    // println!("{:?}", check_right((0,5),6));
    // println!("{:?}", check_up((0,5),6));
    // println!("{:?}", check_down((0,5),6));
    
    let mut pzl_vec:Vec<Vec<u8>> = Vec::new();
    let mut length_here: u8= 0;
    (pzl_vec,length_here)  = puzzle_str2vec(&puzzle);
    
    let mut poss_sol: Vec<(u8,u8)> = generate_coords_loops(length_here);
    println!("{:?}", path_finder(length_here, pzl_vec, poss_sol));
    

}
