//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 1183/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk1183<F: Float>(t19517: F, t584: F, t2349: F, t5484: F, t662: F, t19503: F, t103: F, t100: F, t12774: F, t12795: F, t1447: F, t19489: F, t19493: F, t19499: F, t19504: F, t19514: F, t4060: F, t4064: F, t5469: F, t5472: F, t5475: F, t657: F, t663: F, t92: F) -> F {
    let t19518 = t19517 * t584;
    let t19521 = t2349 * t5484;
    let t19522 = t19521 * t662;
    let t19525 = -t19503;
    let t19526 = t103 * t19525;
    let t19529 = -F::new(50.0) / F::new(27.0) * t657 * t5469 - F::new(10.0) / F::new(27.0) * t92 * t19489 + F::new(20.0) / F::new(9.0) * t12774 * t19493 - F::new(25.0) / F::new(9.0) * t657 * t5472 + F::new(10.0) / F::new(9.0) * t92 * t19499 + F::new(5.0) / F::new(3.0) * t92 * t19504 + F::new(200.0) / F::new(27.0) * t5475 * t663 - F::new(100.0) / F::new(27.0) * t1447 * t4060 + F::new(50.0) / F::new(9.0) * t1447 * t4064 - F::new(10.0) / F::new(27.0) * t100 * t19514 - F::new(20.0) / F::new(9.0) * t12795 * t19518 + F::new(10.0) / F::new(9.0) * t100 * t19522 + F::new(5.0) / F::new(3.0) * t100 * t19526;
    t19529
}
