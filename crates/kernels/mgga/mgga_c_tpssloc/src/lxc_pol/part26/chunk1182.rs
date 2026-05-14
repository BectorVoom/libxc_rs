//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1182/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1182<F: Float>(t81520: F, t82333: F, t3034: F, t336: F, t221: F, t697: F, t1016: F, t3: F, t9258: F, t10121: F, t10140: F, t13487: F, t1877: F, t1914: F, t1915: F, t193: F, t202: F, t23286: F, t23290: F, t23295: F, t2379: F, t2522: F, t2553: F, t2745: F, t2749: F, t4314: F, t46240: F, t46252: F, t46298: F, t46320: F, t46362: F, t6666: F, t6670: F, t776: F, t81525: F, t81539: F, t82307: F, t82312: F, t868: F, t870: F, t9458: F, t9516: F, t9616: F) -> (F, F, F, F, F, F) {
    let t82334 = t81520 + t82333;
    let t82510 = 1.0 / t3034 / t336;
    let t82631 = t221 * t697;
    let t82985 = 1.0 / t3034 / t1016;
    let t83100 = t3 * t9258;
    let t83543 = -18.0 * t4314 * t6670 * t46298 + 18.0 * t2522 * t23295 * t46320 - 3.0 * t1877 * t23290 * t2745 - 18.0 * t2522 * t23290 * t13487 + 18.0 * t4314 * t1915 * t9616 + 9.0 * t2522 * t6666 * t2553 - 9.0 * t2522 * t6670 * t46252 - 9.0 * t2522 * t6670 * t46240 + 6.0 * t1877 * t81539 * t2749 + t193 * t202 * t82307 * t870 + 9.0 * t2522 * t23286 * t776 - 6.0 * t1877 * t82312 * t10140 + 6.0 * t1877 * t23295 * t46362 - 3.0 * t1877 * t81525 * t868 + 6.0 * t193 * t9458 * t1914 * t870 + 18.0 * t4314 * t6666 * t2379 - t1877 * t6670 * t10121 + 3.0 * t2522 * t1915 * t9516;
    (t82334, t82510, t82631, t82985, t83100, t83543)
}
