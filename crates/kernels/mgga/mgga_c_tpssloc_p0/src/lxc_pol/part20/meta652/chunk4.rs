//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2404/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2404<F: Float>(t49127: F, t49140: F, t49154: F, t49167: F, t49181: F, t49194: F, t49208: F, t49219: F, t1556: F, t2842: F, t10727: F, t10702: F) -> (F, F, F) {
    let t49222 = t49127 + t49140 + t49154 + t49167 + t49181 + t49194 + t49208 + t49219;
    let t49226 = t2842 * t1556;
    let t49228 = F::new(18.0) * t49226 * t10727;
    let t49240 = t10702 * t1556;
    (t49222, t49228, t49240)
}
