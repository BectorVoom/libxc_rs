//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1109/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1109<F: Float>(t15899: F, t8493: F, t1983: F, t1441: F, t8319: F, t510: F, t1774: F, t8320: F, t7468: F, t8526: F, t12571: F, t8301: F) -> (F, F, F, F, F, F, F) {
    let t33082 = t8493 * t15899;
    let t33084 = F::new(2.0) * t1983 * t33082;
    let t33094 = t1441 * t8319;
    let t33096 = F::new(2.0) * t33094 * t510;
    let t33098 = F::new(2.0) * t8320 * t1774;
    let t33100 = F::new(4.0) * t8526 * t7468;
    let t33103 = t12571 * t8301;
    (t33082, t33084, t33094, t33096, t33098, t33100, t33103)
}
