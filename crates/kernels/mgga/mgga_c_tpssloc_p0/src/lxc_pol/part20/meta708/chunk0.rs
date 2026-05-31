//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2710/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2710<F: Float>(t45616: F, t45648: F, t53774: F, t55315: F, t112: F, t16506: F, t1395: F, t2319: F, t111: F, t5363: F, t12521: F, t12524: F, t12529: F, t12532: F, t12813: F, t1401: F, t1458: F, t16521: F, t16524: F, t16535: F, t16538: F, t16541: F, t1851: F, t20173: F, t2363: F, t3938: F, t3941: F, t4072: F, t45557: F, t45560: F, t45782: F, t5371: F, t5376: F, t577: F, t671: F, t9416: F) -> (F, F) {
    let t55317 = t45616 + t45648 + t53774 + t55315;
    let t55341 = t16506 * t112;
    let t55344 = t1395 * t2319;
    let t55353 = t5363 * t111;
    let t55364 = F::cast_from(0.45e1_f64) * t55317 * t577 + F::cast_from(0.405e2_f64) * t12521 * t4072 + F::cast_from(81.0_f64) * t45560 * t5376 + F::cast_from(27.0_f64) * t3941 * t1458 * t9416 + F::cast_from(0.405e2_f64) * t3938 * t12813 + F::cast_from(81.0_f64) * t16524 * t12532 + F::cast_from(81.0_f64) * t3941 * t12813 * t671 + F::cast_from(81.0_f64) * t3941 * t4072 * t2363 + F::cast_from(81.0_f64) * t20173 * t16541 + F::cast_from(81.0_f64) * t16535 * t4072 + F::cast_from(0.405e2_f64) * t55341 * t671 + F::cast_from(81.0_f64) * t55344 * t1458 + F::cast_from(162.0_f64) * t12524 * t16538 + F::cast_from(81.0_f64) * t12524 * t16541 + F::cast_from(0.405e2_f64) * t16521 * t2363 + F::cast_from(81.0_f64) * t55353 * t2319 + F::cast_from(0.135e2_f64) * t5371 * t9416 + F::cast_from(0.135e2_f64) * t45557 * t1458 + F::cast_from(0.135e2_f64) * t1401 * t45782 + F::cast_from(27.0_f64) * t1851 * t12529;
    (t55317, t55364)
}
