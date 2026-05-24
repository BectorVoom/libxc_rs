//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 872/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk872<F: Float>(t4961: F, t702: F, t2265: F, t5321: F, t41789: F, t41791: F, t41811: F, t41813: F, t1562: F, t8188: F, t41817: F, t41821: F) -> (F, F, F, F, F, F, F, F, F) {
    let t43817 = t4961 * t702;
    let t43836 = F::new(0.4726e1) * t5321 * t2265;
    let t43844 = F::cast_from(0.3193131120497015617e0_f64) * t41789;
    let t43850 = F::cast_from(0.3193131120497015617e0_f64) * t41791;
    let t43861 = F::cast_from(0.39726959900411316772e-4_f64) * t41811;
    let t43862 = F::cast_from(0.11918087970123395032e-3_f64) * t41813;
    let t43864 = F::new(0.4726e1) * t1562 * t8188;
    let t43868 = F::cast_from(0.1440846329149835838e-2_f64) * t41817;
    let t43869 = F::cast_from(0.1440846329149835838e-2_f64) * t41821;
    (t43817, t43836, t43844, t43850, t43861, t43862, t43864, t43868, t43869)
}
