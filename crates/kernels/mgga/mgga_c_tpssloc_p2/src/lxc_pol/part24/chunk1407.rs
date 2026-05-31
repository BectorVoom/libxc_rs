//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1407/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1407<F: Float>(t23734: F, t3216: F, t11094: F, t6818: F, t1958: F, t43637: F, t10121: F, t10140: F, t13487: F, t1877: F, t1914: F, t1915: F, t193: F, t202: F, t23286: F, t23290: F, t23295: F, t2379: F, t2522: F, t2553: F, t2745: F, t2749: F, t4314: F, t46240: F, t46252: F, t46298: F, t46320: F, t46362: F, t6666: F, t6670: F, t776: F, t81525: F, t81539: F, t82307: F, t82312: F, t868: F, t870: F, t9458: F, t9516: F, t9616: F) -> (F, F, F, F) {
    let t83468 = t23734 * t3216;
    let t83472 = t6818 * t11094;
    let t83479 = t1958 * t43637;
    let t83543 = -F::cast_from(18.0_f64) * t4314 * t6670 * t46298 + F::cast_from(18.0_f64) * t2522 * t23295 * t46320 - F::cast_from(3.0_f64) * t1877 * t23290 * t2745 - F::cast_from(18.0_f64) * t2522 * t23290 * t13487 + F::cast_from(18.0_f64) * t4314 * t1915 * t9616 + F::cast_from(9.0_f64) * t2522 * t6666 * t2553 - F::cast_from(9.0_f64) * t2522 * t6670 * t46252 - F::cast_from(9.0_f64) * t2522 * t6670 * t46240 + F::cast_from(6.0_f64) * t1877 * t81539 * t2749 + t193 * t202 * t82307 * t870 + F::cast_from(9.0_f64) * t2522 * t23286 * t776 - F::cast_from(6.0_f64) * t1877 * t82312 * t10140 + F::cast_from(6.0_f64) * t1877 * t23295 * t46362 - F::cast_from(3.0_f64) * t1877 * t81525 * t868 + F::cast_from(6.0_f64) * t193 * t9458 * t1914 * t870 + F::cast_from(18.0_f64) * t4314 * t6666 * t2379 - t1877 * t6670 * t10121 + F::cast_from(3.0_f64) * t2522 * t1915 * t9516;
    (t83468, t83472, t83479, t83543)
}
