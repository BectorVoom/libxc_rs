//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 590/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk590<F: Float>(t1804: F, t3726: F, t131: F, t3732: F, t205: F, t1799: F, t213: F, t1307: F, t221: F, t118: F, t794: F, t3739: F, t210: F, t214: F, t5187: F, t1315: F, t3725: F, t3727: F, t3731: F, t3742: F, t3751: F) -> (F,) {
    let t5192 = t3726 * t1804;
    let t5194 = t3732 * t131;
    let t5195 = t205 * t5194;
    let t5196 = t213 * t1799;
    let t5198 = t221 * t5196 * t1307;
    let t5202 = t118 * t794 * t1799;
    let t5203 = t3739 * t5202;
    let t5206 = t210 * t214 * t5187;
    let t5210 = t3725 + 0.38888888888888888888e-2 * t3727 + t3731 + 0.38888888888888888887e-2 * t5192 + 0.49999999999999999998e-2 * t5195 * t5198 + 0.8333333333333333333e-3 * t5203 - 0.16666666666666666666e-2 * t1315 * t5206 + 0.83333333333333333332e-3 * t3742 - t3751;
    (t5210,)
}
