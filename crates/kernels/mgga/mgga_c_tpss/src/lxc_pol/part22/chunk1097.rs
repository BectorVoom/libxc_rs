//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1097/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1097<F: Float>(t11844: F, t11873: F, t11857: F, t11860: F, t11862: F, t11865: F, t11867: F, t11871: F, t11875: F, t11880: F, t11885: F, t11890: F) -> (F, F) {
    let t12024 = F::cast_from(0.13892666666666666667e0_f64) * t11844;
    let t12035 = F::cast_from(0.22954444444444444444e0_f64) * t11873;
    let t12040 = -F::cast_from(0.157790625e0_f64) * t11857 - F::new(0.3529725e1) * t11860 - F::new(0.17648625e1) * t11862 + F::new(0.6311625e0) * t11865 + F::new(0.31558125e0) * t11867 + F::new(0.62517e0) * t11871 + t12035 - F::cast_from(0.68863333333333333333e0_f64) * t11875 + F::cast_from(0.57386111111111111112e0_f64) * t11880 - F::new(0.20659e1) * t11885 - F::cast_from(0.68863333333333333334e0_f64) * t11890;
    (t12024, t12040)
}
