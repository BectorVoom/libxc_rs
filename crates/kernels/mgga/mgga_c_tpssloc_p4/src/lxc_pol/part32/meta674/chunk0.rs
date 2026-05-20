//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2109/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2109<F: Float>(t27604: F, t3523: F, t24683: F, t24746: F, t8027: F, t4928: F, t52: F, t2132: F, t8040: F, t86292: F, t15689: F, t7310: F) -> (F, F, F, F, F, F) {
    let t95465 = t27604 * t3523 / F::new(324.0);
    let t95480 = F::cast_from(0.16149102437656156342e-2_f64) * t8027 * t24683 * t24746;
    let t95484 = t52 * t4928;
    let t95487 = F::cast_from(0.20186378047070195428e-3_f64) * t2132 * t95484 * t24746;
    let t95491 = F::cast_from(0.20186378047070195428e-3_f64) * t86292 * t8040;
    let t95507 = t7310 * t15689 / F::new(432.0);
    (t95465, t95480, t95484, t95487, t95491, t95507)
}
