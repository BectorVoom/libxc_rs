//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 759/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk759<F: Float>(t35607: F, t1343: F, t2019: F, t2084: F, t7765: F, t1330: F, t28: F, t271: F, t7553: F, t7557: F, t7491: F, t7766: F) -> (F, F, F, F) {
    let t35608 = F::cast_from(0.45731474687362542471e-3_f64) * t35607;
    let t35611 = t2019 * t2084 * t1343 * t7765;
    let t35612 = F::cast_from(0.24390119833260022651e-2_f64) * t35611;
    let t35613 = t28 * t1330;
    let t35616 = t7553 * t35613 * t271 * t7557;
    let t35617 = F::cast_from(0.5854811038705731867e-3_f64) * t35616;
    let t35618 = t7491 * t7766;
    (t35608, t35612, t35617, t35618)
}
