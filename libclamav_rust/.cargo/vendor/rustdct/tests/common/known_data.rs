pub struct KnownTestValues {
    pub input: Vec<f64>,
    pub expected_output: Vec<f64>,
}
// [Autogenerated Test Tata]
// known input/output values for DCT Type 1
pub fn known_values_dct1() -> Vec<KnownTestValues> {
    vec![
        KnownTestValues {
            input: vec![0.0, 0.0],
            expected_output: vec![0.0, 0.0],
        },
        KnownTestValues {
            input: vec![0.0, 0.0, 0.0],
            expected_output: vec![0.0, 0.0, 0.0],
        },
        KnownTestValues {
            input: vec![0.0, 0.0, 0.0, 0.0],
            expected_output: vec![0.0, 0.0, 0.0, 0.0],
        },
        KnownTestValues {
            input: vec![0.0, 0.0, 0.0, 0.0, 0.0],
            expected_output: vec![0.0, 0.0, 0.0, 0.0, 0.0],
        },
        KnownTestValues {
            input: vec![1.0, 1.0],
            expected_output: vec![1.0, 0.0],
        },
        KnownTestValues {
            input: vec![1.0, 1.0, 1.0],
            expected_output: vec![2.0, 0.0, 0.0],
        },
        KnownTestValues {
            input: vec![1.0, 1.0, 1.0, 1.0],
            expected_output: vec![3.0, 0.0, 0.0, 0.0],
        },
        KnownTestValues {
            input: vec![1.0, 1.0, 1.0, 1.0, 1.0],
            expected_output: vec![4.0, 0.0, 0.0, 0.0, 0.0],
        },
        KnownTestValues {
            input: vec![3.3, 5.9],
            expected_output: vec![4.6, -1.3],
        },
        KnownTestValues {
            input: vec![3.3, 5.9, -1.9],
            expected_output: vec![6.6, 2.6, -5.2],
        },
        KnownTestValues {
            input: vec![3.3, 5.9, -1.9, 6.25],
            expected_output: vec![8.775, 2.425, 2.775, -9.275],
        },
        KnownTestValues {
            input: vec![3.3, 5.9, -1.9, 6.25, 0.6],
            expected_output: vec![12.2, 1.1025, 3.85, 1.5975, -12.1],
        },
    ]
}

// known input/output values for DCT Type 2
pub fn known_values_dct2() -> Vec<KnownTestValues> {
    vec![
        KnownTestValues {
            input: vec![0.0],
            expected_output: vec![0.0],
        },
        KnownTestValues {
            input: vec![0.0, 0.0],
            expected_output: vec![0.0, 0.0],
        },
        KnownTestValues {
            input: vec![0.0, 0.0, 0.0],
            expected_output: vec![0.0, 0.0, 0.0],
        },
        KnownTestValues {
            input: vec![0.0, 0.0, 0.0, 0.0],
            expected_output: vec![0.0, 0.0, 0.0, 0.0],
        },
        KnownTestValues {
            input: vec![0.0, 0.0, 0.0, 0.0, 0.0],
            expected_output: vec![0.0, 0.0, 0.0, 0.0, 0.0],
        },
        KnownTestValues {
            input: vec![1.0],
            expected_output: vec![1.0],
        },
        KnownTestValues {
            input: vec![1.0, 1.0],
            expected_output: vec![2.0, 0.0],
        },
        KnownTestValues {
            input: vec![1.0, 1.0, 1.0],
            expected_output: vec![3.0, 0.0, 0.0],
        },
        KnownTestValues {
            input: vec![1.0, 1.0, 1.0, 1.0],
            expected_output: vec![4.0, 0.0, 0.0, 0.0],
        },
        KnownTestValues {
            input: vec![1.0, 1.0, 1.0, 1.0, 1.0],
            expected_output: vec![5.0, 0.0, 0.0, 0.0, 0.0],
        },
        KnownTestValues {
            input: vec![3.3],
            expected_output: vec![3.3],
        },
        KnownTestValues {
            input: vec![3.3, 5.9],
            expected_output: vec![9.2, -1.8385],
        },
        KnownTestValues {
            input: vec![3.3, 5.9, -1.9],
            expected_output: vec![7.3, 4.5033, -5.2],
        },
        KnownTestValues {
            input: vec![3.3, 5.9, -1.9, 6.25],
            expected_output: vec![13.55, 0.25949, 3.9244, -8.3352],
        },
        KnownTestValues {
            input: vec![3.3, 5.9, -1.9, 6.25, 0.6],
            expected_output: vec![14.15, 2.3621, 1.3006, 1.9199, -10.524],
        },
    ]
}

// known input/output values for DCT Type 3
pub fn known_values_dct3() -> Vec<KnownTestValues> {
    vec![
        KnownTestValues {
            input: vec![0.0],
            expected_output: vec![0.0],
        },
        KnownTestValues {
            input: vec![0.0, 0.0],
            expected_output: vec![0.0, 0.0],
        },
        KnownTestValues {
            input: vec![0.0, 0.0, 0.0],
            expected_output: vec![0.0, 0.0, 0.0],
        },
        KnownTestValues {
            input: vec![0.0, 0.0, 0.0, 0.0],
            expected_output: vec![0.0, 0.0, 0.0, 0.0],
        },
        KnownTestValues {
            input: vec![0.0, 0.0, 0.0, 0.0, 0.0],
            expected_output: vec![0.0, 0.0, 0.0, 0.0, 0.0],
        },
        KnownTestValues {
            input: vec![1.0],
            expected_output: vec![0.5],
        },
        KnownTestValues {
            input: vec![1.0, 1.0],
            expected_output: vec![1.2071, -0.20711],
        },
        KnownTestValues {
            input: vec![1.0, 1.0, 1.0],
            expected_output: vec![1.866, -0.5, 0.13397],
        },
        KnownTestValues {
            input: vec![1.0, 1.0, 1.0, 1.0],
            expected_output: vec![2.5137, -0.7483, 0.33409, -0.099456],
        },
        KnownTestValues {
            input: vec![1.0, 1.0, 1.0, 1.0, 1.0],
            expected_output: vec![3.1569, -0.98131, 0.5, -0.25476, 0.079192],
        },
        KnownTestValues {
            input: vec![3.3],
            expected_output: vec![1.65],
        },
        KnownTestValues {
            input: vec![3.3, 5.9],
            expected_output: vec![5.8219, -2.5219],
        },
        KnownTestValues {
            input: vec![3.3, 5.9, -1.9],
            expected_output: vec![5.8095, 3.55, -4.4095],
        },
        KnownTestValues {
            input: vec![3.3, 5.9, -1.9, 6.25],
            expected_output: vec![8.1492, -0.52291, 6.5099, -7.5362],
        },
        KnownTestValues {
            input: vec![3.3, 5.9, -1.9, 6.25, 0.6],
            expected_output: vec![9.5832, -0.72445, 4.15, 4.2279, -8.9866],
        },
    ]
}

// known input/output values for DCT Type 4
pub fn known_values_dct4() -> Vec<KnownTestValues> {
    vec![
        KnownTestValues {
            input: vec![0.0],
            expected_output: vec![0.0],
        },
        KnownTestValues {
            input: vec![0.0, 0.0],
            expected_output: vec![0.0, 0.0],
        },
        KnownTestValues {
            input: vec![0.0, 0.0, 0.0],
            expected_output: vec![0.0, 0.0, 0.0],
        },
        KnownTestValues {
            input: vec![0.0, 0.0, 0.0, 0.0],
            expected_output: vec![0.0, 0.0, 0.0, 0.0],
        },
        KnownTestValues {
            input: vec![0.0, 0.0, 0.0, 0.0, 0.0],
            expected_output: vec![0.0, 0.0, 0.0, 0.0, 0.0],
        },
        KnownTestValues {
            input: vec![1.0],
            expected_output: vec![0.70711],
        },
        KnownTestValues {
            input: vec![1.0, 1.0],
            expected_output: vec![1.3066, -0.5412],
        },
        KnownTestValues {
            input: vec![1.0, 1.0, 1.0],
            expected_output: vec![1.9319, -0.70711, 0.51764],
        },
        KnownTestValues {
            input: vec![1.0, 1.0, 1.0, 1.0],
            expected_output: vec![2.5629, -0.89998, 0.60134, -0.5098],
        },
        KnownTestValues {
            input: vec![1.0, 1.0, 1.0, 1.0, 1.0],
            expected_output: vec![3.1962, -1.1013, 0.70711, -0.56116, 0.50623],
        },
        KnownTestValues {
            input: vec![3.3],
            expected_output: vec![2.3335],
        },
        KnownTestValues {
            input: vec![3.3, 5.9],
            expected_output: vec![5.3066, -4.188],
        },
        KnownTestValues {
            input: vec![3.3, 5.9, -1.9],
            expected_output: vec![6.8677, -0.49497, -5.1531],
        },
        KnownTestValues {
            input: vec![3.3, 5.9, -1.9, 6.25],
            expected_output: vec![8.306, -0.016005, 0.87276, -10.344],
        },
        KnownTestValues {
            input: vec![3.3, 5.9, -1.9, 6.25, 0.6],
            expected_output: vec![10.104, -1.2387, 4.3487, -5.2296, -8.482],
        },
    ]
}

// known input/output values for DST Type 1
pub fn known_values_dst1() -> Vec<KnownTestValues> {
    vec![
        KnownTestValues {
            input: vec![0.0],
            expected_output: vec![0.0],
        },
        KnownTestValues {
            input: vec![0.0, 0.0],
            expected_output: vec![0.0, 0.0],
        },
        KnownTestValues {
            input: vec![0.0, 0.0, 0.0],
            expected_output: vec![0.0, 0.0, 0.0],
        },
        KnownTestValues {
            input: vec![0.0, 0.0, 0.0, 0.0],
            expected_output: vec![0.0, 0.0, 0.0, 0.0],
        },
        KnownTestValues {
            input: vec![0.0, 0.0, 0.0, 0.0, 0.0],
            expected_output: vec![0.0, 0.0, 0.0, 0.0, 0.0],
        },
        KnownTestValues {
            input: vec![1.0],
            expected_output: vec![1.0],
        },
        KnownTestValues {
            input: vec![1.0, 1.0],
            expected_output: vec![1.7321, 0.0],
        },
        KnownTestValues {
            input: vec![1.0, 1.0, 1.0],
            expected_output: vec![2.4142, 0.0, 0.41421],
        },
        KnownTestValues {
            input: vec![1.0, 1.0, 1.0, 1.0],
            expected_output: vec![3.0777, 0.0, 0.72654, 0.0],
        },
        KnownTestValues {
            input: vec![1.0, 1.0, 1.0, 1.0, 1.0],
            expected_output: vec![3.7321, 0.0, 1.0, 0.0, 0.26795],
        },
        KnownTestValues {
            input: vec![3.3],
            expected_output: vec![3.3],
        },
        KnownTestValues {
            input: vec![3.3, 5.9],
            expected_output: vec![7.9674, -2.2517],
        },
        KnownTestValues {
            input: vec![3.3, 5.9, -1.9],
            expected_output: vec![6.8899, 5.2, -4.9101],
        },
        KnownTestValues {
            input: vec![3.3, 5.9, -1.9, 6.25],
            expected_output: vec![9.4176, 1.7791, 6.7314, -9.1522],
        },
        KnownTestValues {
            input: vec![3.3, 5.9, -1.9, 6.25, 0.6],
            expected_output: vec![10.572, 2.0352, 5.8, 2.6414, -10.472],
        },
    ]
}

// known input/output values for DST Type 2
pub fn known_values_dst2() -> Vec<KnownTestValues> {
    vec![
        KnownTestValues {
            input: vec![0.0],
            expected_output: vec![0.0],
        },
        KnownTestValues {
            input: vec![0.0, 0.0],
            expected_output: vec![0.0, 0.0],
        },
        KnownTestValues {
            input: vec![0.0, 0.0, 0.0],
            expected_output: vec![0.0, 0.0, 0.0],
        },
        KnownTestValues {
            input: vec![0.0, 0.0, 0.0, 0.0],
            expected_output: vec![0.0, 0.0, 0.0, 0.0],
        },
        KnownTestValues {
            input: vec![0.0, 0.0, 0.0, 0.0, 0.0],
            expected_output: vec![0.0, 0.0, 0.0, 0.0, 0.0],
        },
        KnownTestValues {
            input: vec![1.0],
            expected_output: vec![1.0],
        },
        KnownTestValues {
            input: vec![1.0, 1.0],
            expected_output: vec![1.4142, 0.0],
        },
        KnownTestValues {
            input: vec![1.0, 1.0, 1.0],
            expected_output: vec![2.0, 0.0, 1.0],
        },
        KnownTestValues {
            input: vec![1.0, 1.0, 1.0, 1.0],
            expected_output: vec![2.6131, 0.0, 1.0824, 0.0],
        },
        KnownTestValues {
            input: vec![1.0, 1.0, 1.0, 1.0, 1.0],
            expected_output: vec![3.2361, 0.0, 1.2361, 0.0, 1.0],
        },
        KnownTestValues {
            input: vec![3.3],
            expected_output: vec![3.3],
        },
        KnownTestValues {
            input: vec![3.3, 5.9],
            expected_output: vec![6.5054, -2.6],
        },
        KnownTestValues {
            input: vec![3.3, 5.9, -1.9],
            expected_output: vec![6.6, 4.5033, -4.5],
        },
        KnownTestValues {
            input: vec![3.3, 5.9, -1.9, 6.25],
            expected_output: vec![7.3501, 3.4295, 7.2923, -10.75],
        },
        KnownTestValues {
            input: vec![3.3, 5.9, -1.9, 6.25, 0.6],
            expected_output: vec![9.1347, 1.2542, 8.8097, 2.7736, -10.15],
        },
    ]
}

// known input/output values for DST Type 3
pub fn known_values_dst3() -> Vec<KnownTestValues> {
    vec![
        KnownTestValues {
            input: vec![0.0],
            expected_output: vec![0.0],
        },
        KnownTestValues {
            input: vec![0.0, 0.0],
            expected_output: vec![0.0, 0.0],
        },
        KnownTestValues {
            input: vec![0.0, 0.0, 0.0],
            expected_output: vec![0.0, 0.0, 0.0],
        },
        KnownTestValues {
            input: vec![0.0, 0.0, 0.0, 0.0],
            expected_output: vec![0.0, 0.0, 0.0, 0.0],
        },
        KnownTestValues {
            input: vec![0.0, 0.0, 0.0, 0.0, 0.0],
            expected_output: vec![0.0, 0.0, 0.0, 0.0, 0.0],
        },
        KnownTestValues {
            input: vec![1.0],
            expected_output: vec![0.5],
        },
        KnownTestValues {
            input: vec![1.0, 1.0],
            expected_output: vec![1.2071, 0.20711],
        },
        KnownTestValues {
            input: vec![1.0, 1.0, 1.0],
            expected_output: vec![1.866, 0.5, 0.13397],
        },
        KnownTestValues {
            input: vec![1.0, 1.0, 1.0, 1.0],
            expected_output: vec![2.5137, 0.7483, 0.33409, 0.099456],
        },
        KnownTestValues {
            input: vec![1.0, 1.0, 1.0, 1.0, 1.0],
            expected_output: vec![3.1569, 0.98131, 0.5, 0.25476, 0.079192],
        },
        KnownTestValues {
            input: vec![3.3],
            expected_output: vec![1.65],
        },
        KnownTestValues {
            input: vec![3.3, 5.9],
            expected_output: vec![5.2835, -0.61655],
        },
        KnownTestValues {
            input: vec![3.3, 5.9, -1.9],
            expected_output: vec![5.8095, 4.25, -4.4095],
        },
        KnownTestValues {
            input: vec![3.3, 5.9, -1.9, 6.25],
            expected_output: vec![6.8044, 4.8228, 2.729, -7.7894],
        },
        KnownTestValues {
            input: vec![3.3, 5.9, -1.9, 6.25, 0.6],
            expected_output: vec![9.1947, 3.7202, 5.5, -0.15495, -9.6294],
        },
    ]
}

// known input/output values for DST Type 4
pub fn known_values_dst4() -> Vec<KnownTestValues> {
    vec![
        KnownTestValues {
            input: vec![0.0],
            expected_output: vec![0.0],
        },
        KnownTestValues {
            input: vec![0.0, 0.0],
            expected_output: vec![0.0, 0.0],
        },
        KnownTestValues {
            input: vec![0.0, 0.0, 0.0],
            expected_output: vec![0.0, 0.0, 0.0],
        },
        KnownTestValues {
            input: vec![0.0, 0.0, 0.0, 0.0],
            expected_output: vec![0.0, 0.0, 0.0, 0.0],
        },
        KnownTestValues {
            input: vec![0.0, 0.0, 0.0, 0.0, 0.0],
            expected_output: vec![0.0, 0.0, 0.0, 0.0, 0.0],
        },
        KnownTestValues {
            input: vec![1.0],
            expected_output: vec![0.70711],
        },
        KnownTestValues {
            input: vec![1.0, 1.0],
            expected_output: vec![1.3066, 0.5412],
        },
        KnownTestValues {
            input: vec![1.0, 1.0, 1.0],
            expected_output: vec![1.9319, 0.70711, 0.51764],
        },
        KnownTestValues {
            input: vec![1.0, 1.0, 1.0, 1.0],
            expected_output: vec![2.5629, 0.89998, 0.60134, 0.5098],
        },
        KnownTestValues {
            input: vec![1.0, 1.0, 1.0, 1.0, 1.0],
            expected_output: vec![3.1962, 1.1013, 0.70711, 0.56116, 0.50623],
        },
        KnownTestValues {
            input: vec![3.3],
            expected_output: vec![2.3335],
        },
        KnownTestValues {
            input: vec![3.3, 5.9],
            expected_output: vec![6.7137, 0.79097],
        },
        KnownTestValues {
            input: vec![3.3, 5.9, -1.9],
            expected_output: vec![3.1908, 7.8489, -1.4761],
        },
        KnownTestValues {
            input: vec![3.3, 5.9, -1.9, 6.25],
            expected_output: vec![8.4718, 2.0527, 9.2307, -3.944],
        },
        KnownTestValues {
            input: vec![3.3, 5.9, -1.9, 6.25, 0.6],
            expected_output: vec![8.0127, 4.4697, 3.8537, 9.2615, -6.0846],
        },
    ]
}
