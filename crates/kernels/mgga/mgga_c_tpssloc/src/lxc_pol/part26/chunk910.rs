//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 910/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk910<F: Float>(t2840: F, t287: F, t275: F, t2793: F, t912: F, t2844: F, t10294: F, t10544: F, t10296: F, t10298: F, t10300: F, t10302: F, t10307: F, t10314: F, t10320: F, t10323: F, t10530: F, t10538: F, t10547: F, t10550: F) -> (F, F, F) {
    let t10660 = F::new(1.0) / t2840 / t287;
    let t10661 = t275 * t10660;
    let t10662 = t2793 * t912;
    let t10663 = t10662 * t2844;
    let t10665 = F::cast_from(0.96491876992155210402e2_f64) * t10661 * t10663;
    let t10675 = F::cast_from(0.36514074074074074075e0_f64) * t10294;
    let t10676 = F::cast_from(0.93011851851851851854e0_f64) * t10544;
    let t10680 = -F::cast_from(0.59793333333333333333e0_f64) * t10530 - F::cast_from(0.27385555555555555556e0_f64) * t10296 + F::cast_from(0.16431333333333333333e0_f64) * t10302 + F::cast_from(0.5477111111111111111e-1_f64) * t10298 - F::cast_from(0.36514074074074074075e-1_f64) * t10307 - F::cast_from(0.82156666666666666667e-1_f64) * t10323 + F::new(0.17938e1) * t10538 - F::cast_from(0.82156666666666666668e-1_f64) * t10314 + F::cast_from(0.49293999999999999999e0_f64) * t10320 - t10675 - t10676 - F::new(0.28483875e1) * t10547 + F::new(0.46074375e0) * t10550 - F::cast_from(0.32862666666666666666e0_f64) * t10300;
    (t10662, t10665, t10680)
}
