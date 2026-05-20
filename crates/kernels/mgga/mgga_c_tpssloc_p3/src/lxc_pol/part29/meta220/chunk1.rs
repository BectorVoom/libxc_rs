//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1054/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1054<F: Float>(t4930: F, t974: F, t457: F, t1184: F, t1714: F, t460: F, t1174: F, t1180: F, t1187: F, t3430: F, t3433: F, t3436: F, t3447: F, t4887: F, t4889: F, t4897: F, t4901: F, t4905: F, t4909: F, t4913: F, t4917: F, t4920: F) -> (F, F, F, F) {
    let t4931 = t974 * t4930;
    let t4934 = t974 * t457;
    let t4935 = t1714 * t1184;
    let t4936 = t4935 * t460;
    let t4937 = t4934 * t4936;
    let t4940 = -F::cast_from(0.74074074074074074073e-3_f64) * t4887 + F::cast_from(0.74074074074074074073e-3_f64) * t4889 * t1180 + F::cast_from(0.22222222222222222222e-2_f64) * t4889 * t1187 - t3430 - F::cast_from(0.9259259259259259259e-4_f64) * t3433 - F::cast_from(0.27777777777777777777e-3_f64) * t3436 - F::cast_from(0.9259259259259259259e-4_f64) * t4897 + F::cast_from(0.37037037037037037036e-3_f64) * t3447 * t4901 + F::cast_from(0.27777777777777777777e-3_f64) * t3447 * t4905 - F::cast_from(0.55555555555555555554e-3_f64) * t3447 * t4909 - F::cast_from(0.27777777777777777777e-3_f64) * t1174 * t4913 - F::cast_from(0.27777777777777777777e-3_f64) * t4917 + F::cast_from(0.27777777777777777777e-3_f64) * t3447 * t4920 - F::cast_from(0.83333333333333333332e-3_f64) * t1174 * t4931 - F::cast_from(0.83333333333333333332e-3_f64) * t1174 * t4937;
    (t4934, t4935, t4936, t4940)
}
