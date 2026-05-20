//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1301/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1301<F: Float>(t33: F, t9312: F, t2769: F, t73: F, t2291: F, t607: F, t3241: F, t76: F, t2298: F, t2250: F, t634: F, t638: F, t9258: F, t9288: F) -> (F, F, F, F) {
    let t9313 = t33 * t9312;
    let t9321 = F::new(1.0) / t73 / t2769;
    let t9324 = t2291 * t607;
    let t9330 = F::new(1.0) / t76 / t3241;
    let t9333 = t2298 * t607;
    let t9338 = -F::new(280.0) / F::new(27.0) * t9321 * t9288 + F::new(28.0) / F::new(3.0) * t9324 * t2250 - F::new(4.0) / F::new(3.0) * t634 * t9258 + F::new(280.0) / F::new(27.0) * t9330 * t9288 + F::new(28.0) / F::new(3.0) * t9333 * t2250 + F::new(4.0) / F::new(3.0) * t638 * t9258;
    (t9313, t9321, t9330, t9338)
}
