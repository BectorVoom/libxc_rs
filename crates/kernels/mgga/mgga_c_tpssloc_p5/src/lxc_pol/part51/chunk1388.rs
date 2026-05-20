//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1388/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1388<F: Float>(t23185: F, t33457: F, t82074: F, t1888: F, t23270: F, t31332: F, t4300: F, t2048: F, t254: F, t225: F, t33414: F, t1880: F, t23237: F, t33408: F) -> (F, F, F, F, F) {
    let t121444 = t23185 * t82074 * t33457;
    let t121448 = t1888 * t23270 * t31332 * t4300;
    let t121451 = t2048 * t254;
    let t121454 = t33414 * t225;
    let t121457 = t1880 * t23237 * t33408;
    (t121444, t121448, t121451, t121454, t121457)
}
