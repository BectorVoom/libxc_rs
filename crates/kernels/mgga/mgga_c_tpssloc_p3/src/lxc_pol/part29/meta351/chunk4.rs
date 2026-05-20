//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1419/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1419<F: Float>(t12619: F, t72: F, t1410: F, t2283: F, t1426: F, t2244: F, t2251: F, t3997: F, t608: F, t1411: F, t1434: F, t2245: F, t2252: F, t2284: F, t2304: F, t3971: F, t3976: F, t4018: F, t629: F, t642: F, t66: F, t80: F) -> F {
    let t12620 = t72 * t12619;
    let t12623 = t1410 * t2283;
    let t12630 = t2244 * t1426;
    let t12633 = t2251 * t1426;
    let t12636 = t608 * t3997;
    let t12645 = t2284 * t1434 / F::new(24.0) + t629 * t4018 / F::new(12.0) + t66 * t12620 / F::new(24.0) - t12623 * t80 / F::new(12.0) - t3971 * t642 / F::new(6.0) - t1411 * t2304 / F::new(12.0) - t12630 * t80 / F::new(12.0) - t12633 * t80 / F::new(12.0) - t12636 * t80 / F::new(6.0) - t3976 * t642 / F::new(6.0) - t2245 * t1434 / F::new(12.0) - t2252 * t1434 / F::new(12.0);
    t12645
}
