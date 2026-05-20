//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 575/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk575<F: Float>(t33: F, t3997: F, t1409: F, t2291: F, t3966: F, t634: F, t2298: F, t638: F, t607: F, t72: F, t1411: F, t1427: F, t1434: F, t3962: F, t3968: F, t3971: F, t3976: F, t609: F, t629: F, t642: F, t66: F, t80: F) -> (F, F) {
    let t3998 = t33 * t3997;
    let t4007 = t2291 * t1409;
    let t4010 = t634 * t3966;
    let t4012 = t2298 * t1409;
    let t4015 = t638 * t3966;
    let t4017 = F::new(28.0) / F::new(9.0) * t4007 * t607 - F::new(4.0) / F::new(3.0) * t4010 + F::new(28.0) / F::new(9.0) * t4012 * t607 + F::new(4.0) / F::new(3.0) * t4015;
    let t4018 = t72 * t4017;
    let t4021 = -t3962 * t80 / F::new(12.0) - t3968 * t80 / F::new(12.0) - t3971 * t80 / F::new(12.0) - t1411 * t642 / F::new(12.0) - t3976 * t80 / F::new(12.0) + t3998 * t80 / F::new(24.0) + t1427 * t642 / F::new(24.0) - t609 * t1434 / F::new(12.0) + t629 * t1434 / F::new(24.0) + t66 * t4018 / F::new(24.0);
    (t4017, t4021)
}
