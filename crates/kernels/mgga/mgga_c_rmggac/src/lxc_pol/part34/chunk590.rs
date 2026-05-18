//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 590/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk590<F: Float>(t15163: F, t7788: F, t15090: F, t262: F, t7782: F, t15084: F, t7835: F, t15078: F, t793: F, t15049: F, t305: F, t15075: F, t3851: F) -> (F, F, F, F, F, F, F, F) {
    let t15164 = t7788 * t15163;
    let t15166 = t262 * t15090;
    let t15167 = t7782 * t15166;
    let t15169 = t262 * t15084;
    let t15170 = t7835 * t15169;
    let t15172 = t793 * t15078;
    let t15175 = F::new(0.2993560425465952141e-1) * t305 * t15049;
    let t15176 = t3851 * t15075;
    (t15164, t15166, t15167, t15169, t15170, t15172, t15175, t15176)
}
