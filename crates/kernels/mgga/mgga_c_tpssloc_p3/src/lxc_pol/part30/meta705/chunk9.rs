//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2317/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2317<F: Float>(t100025: F, t100068: F, t100103: F, t100147: F, t100176: F, t100195: F, t100225: F, t100253: F, t100287: F, t100314: F, t100341: F, t100377: F, t100396: F, t100430: F, t100459: F, t1052: F, t1055: F, t1603: F, t17875: F, t18070: F, t18074: F, t18166: F, t1945: F, t23581: F, t25705: F, t25755: F, t25757: F, t28499: F, t28679: F, t3169: F, t388: F, t4694: F, t5838: F, t5848: F, t6687: F, t6699: F, t6768: F, t6771: F, t6816: F, t83459: F, t88851: F, t89662: F, t89672: F, t99983: F) -> F {
    let t100489 = -t1052 * t1055 * (t99983 + t100025 + t100068 + t100103 + t100147 + t100176 + t100195 + t100225 + t100253 + t100287 + t100314 + t100341 + t100377 + t100396 + t100430 + t100459) - t89662 - t6771 * t18166 + F::cast_from(0.36554090374405031923e-2_f64) * t89672 + t17875 * t1945 * t388 + t5848 * t6768 * t388 - F::cast_from(0.82246703342411321825e-2_f64) * t6687 * t5838 * t6699 - F::cast_from(0.54831135561607547884e-2_f64) * t6687 * t23581 * t28499 + F::new(2.0) * t1603 * t25705 * t388 - t3169 * t28679 + F::new(24.0) * t25757 * t88851 * t18070 + F::cast_from(0.18277045187202515961e-2_f64) * t83459 - F::new(2.0) * t25755 * t4694 - t18074 * t6816;
    t100489
}
