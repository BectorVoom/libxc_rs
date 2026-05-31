//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1196/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1196<F: Float>(t1409: F, t1634: F, t23330: F, t23329: F, t25442: F, t7553: F, t1955: F, t5943: F, t3174: F, t1052: F, t17575: F, t17588: F, t18074: F, t1956: F, t23327: F, t23359: F, t25807: F, t25824: F, t28594: F, t28679: F, t28681: F, t28684: F, t28691: F, t28697: F, t388: F, t4557: F, t5920: F, t5944: F, t6687: F, t6771: F, t7600: F, t7625: F) -> (F, F, F, F, F) {
    let t28701 = t23330 * t1409 * t1634;
    let t28702 = t23329 * t28701;
    let t28705 = t25442 * t7553;
    let t28712 = t1955 * t5943;
    let t28713 = t3174 * t28712;
    let t28718 = -t6771 * t5944 + t28594 * t388 - t1052 * t28679 - t23359 - F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t28681 - F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t28684 - F::cast_from(2.0_f64) * t17588 * t1956 - F::cast_from(2.0_f64) * t4557 * t7625 - F::cast_from(0.82246703342411321825e-2_f64) * t6687 * t28691 + F::cast_from(4.0_f64) * t4557 * t7600 - F::cast_from(6.0_f64) * t1052 * t28697 - F::cast_from(0.54831135561607547884e-2_f64) * t23327 * t28702 - F::cast_from(0.54831135561607547884e-2_f64) * t23327 * t28705 + F::cast_from(0.54831135561607547884e-2_f64) * t25807 - t17575 * t1956 + F::cast_from(2.0_f64) * t6771 * t5920 + F::cast_from(2.0_f64) * t1052 * t28713 - F::cast_from(0.54831135561607547884e-2_f64) * t25824 - t18074 * t1956;
    (t28701, t28702, t28705, t28713, t28718)
}
