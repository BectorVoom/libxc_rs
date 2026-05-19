//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 917/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk917<F: Float>(t23030: F, t31405: F, t23012: F, t8548: F, t31319: F, t2047: F, t212: F, t23171: F, t6554: F, t23228: F, t8547: F, t193: F, t201: F, t8565: F) -> (F, F, F, F, F, F) {
    let t114814 = t23030 * t31405;
    let t114815 = F::cast_from(0.26044789391763585244e-1_f64) * t114814;
    let t114864 = t23012 * t8548;
    let t114865 = F::cast_from(0.63969658155208805863e-1_f64) * t114864;
    let t114891 = t23030 * t31319;
    let t114892 = F::cast_from(0.26044789391763585244e-1_f64) * t114891;
    let t114932 = t23171 * t212 * t2047 * t6554;
    let t114933 = F::cast_from(0.82246703342411321824e-2_f64) * t114932;
    let t114943 = t23171 * t23228 * t8547;
    let t114944 = F::cast_from(0.82246703342411321824e-2_f64) * t114943;
    let t115009 = t193 * t201 * t8565;
    (t114815, t114865, t114892, t114933, t114944, t115009)
}
