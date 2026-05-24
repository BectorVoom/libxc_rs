//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 850/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk850<F: Float>(t34938: F, t5149: F, t656: F, t1550: F, t2060: F, t27059: F, t2347: F, t876: F, t262: F, t7501: F, t8672: F, t321: F, t8704: F) -> (F, F, F, F, F, F, F) {
    let t39039 = t34938 * t656 * t5149;
    let t39042 = t1550 * t2060 * t27059;
    let t39044 = t2347 * t876;
    let t39045 = t262 * t39044;
    let t39046 = t34938 * t39045;
    let t39048 = t7501 * t8672;
    let t39055 = t8704 * t321;
    (t39039, t39042, t39044, t39045, t39046, t39048, t39055)
}
