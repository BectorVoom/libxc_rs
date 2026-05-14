//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 867/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk867<F: Float>(t25373: F, t25374: F, t1530: F, t606: F, t25: F, t4303: F, t1408: F, t776: F, t868: F, t1877: F, t1915: F, t2219: F, t22959: F, t23290: F, t25013: F, t25015: F, t25021: F, t25024: F, t25028: F, t2522: F, t25354: F, t25358: F, t25366: F, t25372: F, t6542: F, t6666: F, t6670: F, t6671: F, t7475: F, t7541: F, t7545: F) -> (F, F, F, F, F, F, F) {
    let t25375 = t25373 * t25374;
    let t25377 = t606 * t1530;
    let t25381 = t25 * t4303;
    let t25385 = t1408 * t776;
    let t25392 = t1408 * t868;
    let t25397 = t1877 * t1915 * t2219;
    let t25398 = 3.0 * t25013 * t25015 + 3.0 / 2.0 * t2522 * t6666 * t7475 - 3.0 / 2.0 * t22959 * t25021 + 3.0 / 2.0 * t2522 * t1915 * t25024 + 3.0 / 2.0 * t2522 * t1915 * t25028 + 3.0 / 2.0 * t2522 * t7541 * t6542 + t1877 * t25354 * t25 / 2.0 - t1877 * t25358 * t6671 / 2.0 + t1877 * t7541 * t606 / 2.0 - 3.0 / 2.0 * t22959 * t25366 - t1877 * t23290 * t7545 / 2.0 + t25372 * t25375 - t1877 * t6670 * t25377 / 2.0 - t1877 * t6670 * t25381 / 2.0 + 3.0 / 2.0 * t2522 * t1915 * t25385 + t1877 * t6666 * t1408 / 2.0 - t1877 * t6670 * t25392 / 2.0 + t25397;
    (t25375, t25377, t25381, t25385, t25392, t25397, t25398)
}
