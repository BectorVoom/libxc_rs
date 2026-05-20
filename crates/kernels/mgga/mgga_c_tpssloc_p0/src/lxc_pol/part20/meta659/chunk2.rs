//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2454/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2454<F: Float>(t13542: F, t2970: F, t973: F, t13546: F, t10480: F, t13969: F, t13986: F, t3039: F, t4599: F, t49850: F, t10870: F, t4644: F) -> (F, F, F, F, F) {
    let t50242 = t973 * t2970 * t13542;
    let t50250 = t973 * t2970 * t13546;
    let t50255 = t10480 * t13969 * t13986;
    let t50258 = t3039 * t49850 * t4599;
    let t50259 = t50258 / F::new(4608.0);
    let t50262 = t4644 * t10870;
    (t50242, t50250, t50255, t50259, t50262)
}
