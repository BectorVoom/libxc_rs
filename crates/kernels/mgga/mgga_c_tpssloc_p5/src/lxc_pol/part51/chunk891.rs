//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 891/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk891<F: Float>(t3941: F, t8657: F, t2039: F, t577: F, t7010: F, t8508: F, t8646: F, t8654: F, t192: F, t533: F) -> (F, F) {
    let t8659 = F::cast_from(27.0_f64) * t3941 * t8657;
    let t8660 = F::cast_from(0.45e1_f64) * t8646 * t577 + t8654 + F::cast_from(0.135e2_f64) * t7010 * t2039 + t8659 + t8508;
    let t8944 = t192 * t533;
    (t8660, t8944)
}
