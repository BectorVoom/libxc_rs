//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1640/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1640<F: Float>(t5480: F, t9398: F, t662: F, t1449: F, t2: F, t584: F, t2349: F, t5484: F, t19503: F, t103: F, t100: F, t12774: F, t12795: F, t1447: F, t19489: F, t19493: F, t19499: F, t19504: F, t4060: F, t4064: F, t5469: F, t5472: F, t5475: F, t657: F, t663: F, t92: F) -> F {
    let t19513 = t9398 * t5480;
    let t19514 = t19513 * t662;
    let t19517 = t1449 * t2;
    let t19518 = t19517 * t584;
    let t19521 = t2349 * t5484;
    let t19522 = t19521 * t662;
    let t19525 = -t19503;
    let t19526 = t103 * t19525;
    let t19529 = -F::cast_from(50.0_f64) / F::cast_from(27.0_f64) * t657 * t5469 - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t92 * t19489 + F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t12774 * t19493 - F::cast_from(25.0_f64) / F::cast_from(9.0_f64) * t657 * t5472 + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t92 * t19499 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t92 * t19504 + F::cast_from(200.0_f64) / F::cast_from(27.0_f64) * t5475 * t663 - F::cast_from(100.0_f64) / F::cast_from(27.0_f64) * t1447 * t4060 + F::cast_from(50.0_f64) / F::cast_from(9.0_f64) * t1447 * t4064 - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t100 * t19514 - F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t12795 * t19518 + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t100 * t19522 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t100 * t19526;
    t19529
}
