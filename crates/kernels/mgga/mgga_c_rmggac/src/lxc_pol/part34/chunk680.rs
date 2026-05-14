//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 680/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk680<F: Float>(t15214: F, t68528: F, t14116: F, t14117: F, t8496: F, t21713: F, t21714: F, t9054: F, t14125: F, t9095: F, t9137: F, t21718: F, t3352: F, t8516: F, t8518: F, t15318: F, t68432: F) -> (F, F, F, F, F, F, F) {
    let t74065 = t68528 * t15214;
    let t74069 = t14116 * t14117 * t8496;
    let t74072 = t21713 * t21714 * t9054;
    let t74075 = t21713 * t14125 * t9095;
    let t74078 = t21713 * t14125 * t9137;
    let t74082 = t8516 * t21718 * t3352 * t8518;
    let t74084 = t68432 * t15318;
    (t74065, t74069, t74072, t74075, t74078, t74082, t74084)
}
