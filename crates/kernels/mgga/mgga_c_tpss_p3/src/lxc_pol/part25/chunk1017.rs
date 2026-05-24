//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1017/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1017<F: Float>(t14066: F, t3564: F, t189: F, t4579: F, t581: F, t1364: F, t821: F) -> (F, F, F) {
    let t14068 = F::new(24.0) * t3564 * t14066;
    let t14069 = t189 * t4579;
    let t14070 = t14069 * t581;
    let t14072 = F::new(12.0) * t3564 * t14070;
    let t14076 = t1364 * t821;
    (t14068, t14072, t14076)
}
