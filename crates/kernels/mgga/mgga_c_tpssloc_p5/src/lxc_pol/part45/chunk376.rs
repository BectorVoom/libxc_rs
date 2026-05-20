//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 376/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk376<F: Float>(t94: F, t659: F, t2248: F, t95: F, t102: F, t662: F, t103: F, t100: F, t2336: F, t657: F, t660: F, t92: F, t96: F) -> F {
    let t2341 = F::new(1.0) / t94;
    let t2342 = t659 * t659;
    let t2343 = t2341 * t2342;
    let t2346 = t95 * t2248;
    let t2349 = F::new(1.0) / t102;
    let t2350 = t662 * t662;
    let t2351 = t2349 * t2350;
    let t2354 = -t2248;
    let t2355 = t103 * t2354;
    let t2358 = F::new(40.0) / F::new(9.0) * t2336 * t96 - F::new(50.0) / F::new(9.0) * t657 * t660 + F::new(10.0) / F::new(9.0) * t92 * t2343 + F::new(5.0) / F::new(3.0) * t92 * t2346 + F::new(10.0) / F::new(9.0) * t100 * t2351 + F::new(5.0) / F::new(3.0) * t100 * t2355;
    t2358
}
