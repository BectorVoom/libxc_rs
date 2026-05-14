//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1275/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1275<F: Float>(t1864: F, t2241: F, t1863: F, t608: F, t9231: F, t22550: F, t6505: F, t645: F, t6509: F, t1860: F, t1865: F, t22489: F, t22490: F, t22493: F, t22549: F, t22551: F, t6486: F, t6506: F, t6510: F, t83699: F, t83706: F, t83710: F, t83717: F) -> (F,) {
    let t83718 = t1864 * t2241;
    let t83719 = t1863 * t83718;
    let t83722 = t9231 * t608;
    let t83725 = t6505 * t22550;
    let t83728 = t6509 * t645;
    let t83729 = t1863 * t83728;
    let t83732 = t83699 * t1865 - t6486 * t22490 / 2.0 - t1860 * t6505 * t22489 / 2.0 - t1860 * t1863 * t83706 / 6.0 - t83710 * t1865 / 6.0 - t22493 * t6506 / 2.0 - t22493 * t6510 / 2.0 + 30.0 * t83717 * t83719 - 10.0 * t83722 * t22551 - 10.0 * t22549 * t83725 - 10.0 * t22549 * t83729;
    (t83732,)
}
