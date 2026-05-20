//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1417/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1417<F: Float>(t1227: F, t15743: F, t1725: F, t698: F, t1174: F, t225: F, t4941: F, t5053: F, t3701: F, t5356: F, t5168: F, t592: F) -> (F, F, F, F, F, F) {
    let t15745 = F::new(5.0) / F::new(10368.0) * t1227 * t15743;
    let t15753 = t698 * t1725;
    let t15754 = t1174 * t15753;
    let t15797 = t4941 * t225;
    let t15820 = t5053 * t225;
    let t15868 = t5356 * t3701;
    let t15877 = t592 * t5168;
    (t15745, t15754, t15797, t15820, t15868, t15877)
}
