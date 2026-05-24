//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 952/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk952<F: Float>(t3048: F, t8549: F, t8548: F, t3054: F, t9080: F, t1107: F, t3308: F, t8229: F, t1183: F, t123: F, t2349: F, t8220: F) -> (F, F, F, F, F, F) {
    let t9763 = t8549 * t3048;
    let t9764 = t8548 * t9763;
    let t9765 = t9080 * t3054;
    let t9786 = t8549 * t1107;
    let t9787 = t8548 * t9786;
    let t9839 = F::cast_from(0.21687162600603479684e-1_f64) * t3308 * t8229;
    let t9840 = t1183 * t123;
    let t9841 = t9840 * t2349;
    let t9844 = F::cast_from(0.16265371950452609763e-1_f64) * t3308 * t8220;
    (t9764, t9765, t9787, t9839, t9841, t9844)
}
