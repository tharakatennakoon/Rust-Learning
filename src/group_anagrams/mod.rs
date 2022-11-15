use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut output : Vec<Vec<String>> = vec![];
    let mut cmaps : HashMap<String, usize> = HashMap::new();

    for str in strs {
        let mut cs : Vec<char> = str.chars().collect();
        cs.sort();
        let ss = cs.iter().collect();

        match cmaps.get(&ss) {
            Some(index) => {
                let inner_vec = output.get_mut(*index);
                match inner_vec {
                    Some(v) => {
                        v.push(str);
                    },
                    None => {}
                }
            },
            None => {
                let mut inner_vec : Vec<String> = vec![];
                inner_vec.push(str);
                output.push(inner_vec);

                let index = output.len() - 1;

                cmaps.insert(ss, index);
            }
        }
    }

    return output;
}

#[cfg(test)]
mod test_group_anagrams {
    use super::group_anagrams as test_fn;

    #[test]
    fn test1() {
        let input= vec!["eat".to_owned(),"tea".to_owned(),"tan".to_owned(),"ate".to_owned(),"nat".to_owned(),"bat".to_owned()];
        let expected = vec![vec!["eat".to_owned(),"tea".to_owned(),"ate".to_owned()],vec!["tan".to_owned(),"nat".to_owned()],vec!["bat".to_owned()]];

        let output = test_fn(input);

        assert_eq!(expected, output);
    }

    #[test]
    fn test2() {
        let input:Vec<String>= vec![];
        let expected:Vec<Vec<String>> = vec![];

        let output = test_fn(input);

        assert_eq!(expected, output);
    }

    #[test]
    fn test3() {
        let input= vec!["eat".to_owned()];
        let expected = vec![vec!["eat".to_owned()]];

        let output = test_fn(input);

        assert_eq!(expected, output);
    }

}

