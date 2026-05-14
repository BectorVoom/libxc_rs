//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 997/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk997<F: Float>(t1634: F, t8406: F, t3174: F, t1955: F, t7624: F, t225: F, t387: F, t7593: F, t345: F, t1539: F, t30877: F, t6784: F, t1599: F, t8400: F, t6800: F, t7619: F) -> (F, F, F, F, F, F, F, F) {
    let t32912 = t8406 * t1634;
    let t32913 = t3174 * t32912;
    let t32916 = t1955 * t7624;
    let t32917 = t3174 * t32916;
    let t32923 = t7593 * t225 * t387;
    let t32924 = t345 * t32923;
    let t32927 = t30877 * t1539;
    let t32928 = t6784 * t32927;
    let t32931 = t1599 * t8400;
    let t32934 = t7619 * t6800;
    (t32913, t32917, t32923, t32924, t32927, t32928, t32931, t32934)
}
