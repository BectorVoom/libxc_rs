//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 511/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk511<F: Float>(t2932: F, t2951: F, t959: F, t2262: F, t338: F, t964: F, t969: F, t615: F, t972: F) -> (F, F, F, F, F) {
    let t2952 = t2951 * t2932;
    let t2954 = F::new(0.17315859105681463759e2) * t959 * t2952;
    let t2955 = t2262 * t338;
    let t2958 = t964 * t969;
    let t2960 = t615 * t972;
    (t2952, t2954, t2955, t2958, t2960)
}
