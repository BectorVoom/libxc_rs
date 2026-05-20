//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1393/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1393<F: Float>(t23482: F, t23488: F, t23509: F, t23508: F, t6721: F, t6741: F, t1937: F, t23453: F, t40: F, t1000: F, t10195: F, t10250: F, t131: F, t1920: F, t1933: F, t23454: F, t23515: F, t23521: F, t23548: F, t2987: F, t350: F, t4509: F, t6723: F, t6735: F, t6747: F, t82802: F, t83082: F, t83085: F, t83092: F, t83098: F, t83100: F, t83111: F) -> (F, F) {
    let t83114 = t23482 * t23488;
    let t83117 = t23509 * t23488;
    let t83120 = t6721 * t23508;
    let t83121 = t83120 * t6741;
    let t83127 = t23453 * t40 * t1937;
    let t83129 = -t83082 / F::new(72.0) - F::cast_from(0.30279567070605293142e-3_f64) * t83085 * t6747 - F::cast_from(0.21801288290835811062e-1_f64) * t23454 * t6735 + F::cast_from(0.24223653656484234513e-2_f64) * t6723 * t23548 + F::new(11.0) / F::new(108.0) * t83092 * t1000 - F::new(77.0) / F::new(162.0) * t82802 * t131 * t350 + F::new(11.0) / F::new(108.0) * t83098 + F::cast_from(0.10093189023535097714e-3_f64) * t1933 * t83100 * t1937 - t1920 * t2987 * t10250 / F::new(48.0) + t1920 * t4509 * t10195 / F::new(72.0) + F::cast_from(0.21801288290835811062e-1_f64) * t83111 * t6747 - F::cast_from(0.48447307312968469026e-2_f64) * t83114 * t6747 + F::cast_from(0.60559134141210586284e-3_f64) * t83117 * t23515 - F::cast_from(0.48447307312968469026e-2_f64) * t83121 * t23515 + F::cast_from(0.24223653656484234513e-2_f64) * t83121 * t23521 + F::cast_from(0.21801288290835811062e-1_f64) * t83127;
    (t83117, t83129)
}
