//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 747/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk747<F: Float>(t1441: F, t895: F, t1449: F, t903: F, t2455: F, t2513: F, t2601: F, t2608: F, t3746: F, t3751: F, t3756: F, t3760: F, t3774: F, t3782: F, t3790: F, t3792: F, t3795: F, t3798: F, t3801: F, t3804: F) -> (F, F, F) {
    let t3860 = t1441 * t895;
    let t3865 = t1449 * t903;
    let t3882 = -F::cast_from(0.1294625e1_f64) * t3774 + F::cast_from(0.258925e1_f64) * t3782 + t2601 + F::cast_from(0.10064166666666666667e0_f64) * t2455 + F::cast_from(0.10064166666666666667e0_f64) * t3746 - F::cast_from(0.20128333333333333333e0_f64) * t3751 + F::cast_from(0.60385e0_f64) * t3756 - F::cast_from(0.301925e0_f64) * t3760 + F::cast_from(0.82524375e-1_f64) * t3790 + F::cast_from(0.16504875e0_f64) * t3792 + t2608 + F::cast_from(0.5519e-1_f64) * t2513 + F::cast_from(0.5519e-1_f64) * t3795 - F::cast_from(0.27595e-1_f64) * t3798 + F::cast_from(0.16557e0_f64) * t3801 - F::cast_from(0.82785e-1_f64) * t3804;
    (t3860, t3865, t3882)
}
