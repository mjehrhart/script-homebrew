pub mod brew_formula {
    #[derive(Default, Debug, Clone, Copy)]
    pub struct Formula_Rb<'a> {
        pub class_name: &'a str,
        pub desc: &'a str,
        pub homepage: &'a str,
        pub url: &'a str,
        pub version: &'a str,
        pub sha256: &'a str,
        pub license: &'a str,
    }

    

    
}
