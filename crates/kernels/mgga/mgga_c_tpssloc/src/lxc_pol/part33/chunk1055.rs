//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1055/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1055<F: Float>(t1268: F, t28017: F, t1458: F, t24999: F, t27993: F, t27996: F, t28001: F, t28004: F, t28006: F, t28009: F, t28011: F, t5493: F, t6517: F, t510: F, t652: F, t7685: F, t7756: F) -> (F, F, F, F) {
    let t28019 = 2.0 * t1268 * t28017;
    let t28020 = 4.0 * t1458 * t24999 + 2.0 * t5493 * t6517 + t27993 + 2.0 * t27996 + t28001 + t28004 + t28006 + t28009 + t28011 + t28019;
    let t28025 = t510 * t28017;
    let t28027 = 2.0 * t652 * t28025;
    let t28029 = 2.0 * t7685 * t7756;
    (t28020, t28025, t28027, t28029)
}
