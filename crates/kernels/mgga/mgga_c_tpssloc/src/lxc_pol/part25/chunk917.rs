//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 917/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk917<F: Float>(t22724: F, t6898: F, t6902: F, t794: F, t6897: F, t22666: F, t6891: F, t6888: F, t225: F, t3886: F, t3888: F, t6889: F, t1985: F, t6883: F, t6903: F, t2379: F, t25: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t22925 = t22724 * t6898;
    let t22927 = t794 * t6902;
    let t22928 = t6897 * t22927;
    let t22930 = t22666 * t6891;
    let t22931 = t6888 * t22930;
    let t22933 = t225 * t3886;
    let t22934 = t22933 * t3888;
    let t22935 = t6889 * t22934;
    let t22936 = t1985 * t22935;
    let t22940 = t6883 * t6903;
    let t22951 = t25 * t2379;
    (t22925, t22927, t22928, t22930, t22931, t22934, t22935, t22936, t22940, t22951)
}
