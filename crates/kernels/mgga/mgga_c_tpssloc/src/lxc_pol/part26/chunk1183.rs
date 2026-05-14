//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1183/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1183<F: Float>(t1081: F, t2752: F, t13487: F, t10121: F, t28: F, t2379: F, t23788: F, t46240: F, t25927: F, t46320: F, t10140: F, t3231: F, t776: F, t2553: F, t868: F, t1877: F, t1915: F, t22959: F, t23290: F, t23781: F, t23792: F, t23796: F, t23807: F, t23810: F, t2522: F, t4314: F, t6666: F, t6670: F, t81539: F, t82308: F, t82312: F) -> (F,) {
    let t83555 = t2752 * t1081;
    let t83556 = t83555 * t13487;
    let t83559 = t28 * t10121;
    let t83566 = t1081 * t2379;
    let t83579 = t23788 * t46240;
    let t83582 = t25927 * t46320;
    let t83585 = t28 * t10140;
    let t83592 = t3231 * t776;
    let t83596 = t1081 * t2553;
    let t83603 = t3231 * t868;
    let t83607 = -9.0 * t22959 * t83556 - t1877 * t6670 * t83559 / 2.0 + 9.0 * t2522 * t6666 * t23792 + 9.0 * t4314 * t1915 * t83566 + t1877 * t82308 * t28 / 2.0 + 9.0 * t4314 * t6666 * t23781 - 3.0 * t1877 * t23290 * t23810 - 9.0 / 2.0 * t22959 * t83579 + 9.0 * t22959 * t83582 - 3.0 * t1877 * t82312 * t83585 + 3.0 * t1877 * t81539 * t23807 + 9.0 / 2.0 * t2522 * t1915 * t83592 + 9.0 / 2.0 * t2522 * t1915 * t83596 + 9.0 / 2.0 * t2522 * t6666 * t23796 - 3.0 / 2.0 * t1877 * t6670 * t83603;
    (t83607,)
}
