//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 563/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk563<F: Float>(t1338: F, t2006: F, t33: F, t63: F, t2240: F, t625: F, t67: F, t1864: F, t1860: F, t111: F, t2035: F) -> (F, F, F, F, F, F, F) {
    let t6987 = t1338 * t2006;
    let t7025 = t33 * t63;
    let t7026 = t2240 * t7025;
    let t7031 = t625 * t67;
    let t7032 = t7031 * t1864;
    let t7034 = F::new(8.0) / F::new(9.0) * t1860 * t7032;
    let t7042 = t2035 * t111;
    (t6987, t7025, t7026, t7031, t7032, t7034, t7042)
}
