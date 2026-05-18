//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 578/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk578<F: Float>(t2080: F, t2211: F, t739: F, t13964: F, t14092: F, t14108: F, t14152: F, t14269: F, t14364: F, t14369: F, t2339: F, t3056: F, t3057: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t14710 = t2211 * t2080;
    let t14711 = t739 * t14710;
    let t14712 = F::new(0.2993560425465952141e-1) * t14711;
    let t14825 = F::new(0.13010691197123848592e-4) * t13964;
    let t14849 = F::new(0.11400064176674482499e-6) * t14092;
    let t14865 = F::new(0.15965655602485078085e0) * t14108;
    let t14883 = F::new(0.13010691197123848592e-4) * t14152;
    let t14913 = F::new(0.34695176525663596246e-4) * t14269;
    let t14918 = F::new(0.1276937996798935182e-3) * t14364;
    let t14919 = F::new(0.16351352353374609375e-5) * t14369;
    let t15030 = t3056 * t3057 * t2339;
    (t14710, t14712, t14825, t14849, t14865, t14883, t14913, t14918, t14919, t15030)
}
