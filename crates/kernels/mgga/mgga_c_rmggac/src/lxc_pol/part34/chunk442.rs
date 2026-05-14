//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 442/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk442<F: Float>(t13883: F, t1986: F, t3141: F, t3122: F, t4179: F, t7: F, t2003: F, t3133: F, t13862: F, t323: F, t3046: F, t6444: F, t333: F, t3851: F, t2048: F, t793: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t13884 = t1986 * t13883;
    let t13885 = t3141 * t13884;
    let t13888 = t3122 * t7 * t4179;
    let t13889 = t13888 * t2003;
    let t13890 = t3133 * t13889;
    let t13892 = t13862 * t323;
    let t13893 = t3133 * t13892;
    let t13895 = t6444 * t3046;
    let t13897 = t3046 * t333;
    let t13898 = t3851 * t13897;
    let t13900 = t793 * t2048;
    (t13884, t13885, t13888, t13889, t13890, t13892, t13893, t13895, t13897, t13898, t13900)
}
