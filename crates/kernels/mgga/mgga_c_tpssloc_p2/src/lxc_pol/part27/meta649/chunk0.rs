//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2250/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2250<F: Float>(t25836: F, t3216: F, t11094: F, t7627: F, t1068: F, t1070: F, t14662: F, t1637: F, t193: F, t23738: F, t23742: F, t25840: F, t25845: F, t3209: F, t3213: F, t336: F, t4696: F, t4700: F, t60941: F, t6822: F, t83468: F, t83472: F, t83479: F, t88054: F, t88097: F, t88137: F, t88179: F, t88213: F, t88742: F, t88779: F, t88827: F, t88867: F, t88900: F, t88940: F, t89556: F, t89590: F, t89623: F, t89658: F, t89690: F) -> F {
    let t89698 = t25836 * t3216;
    let t89702 = t7627 * t11094;
    let t89729 = t193 * t336 * (t88054 + t88097 + t88137 + t88179 + t88213 + t88742 + t88779 + t88827 + t88867 + t88900 + t88940 + t89556 + t89590 + t89623 + t89658 + t89690) * t1070 - F::cast_from(2.0_f64) * t4700 * t89698 * t1068 + F::cast_from(2.0_f64) * t4700 * t89702 * t3213 - t4700 * t25840 * t3209 - t4700 * t83468 * t1637 + F::cast_from(4.0_f64) * t4700 * t83472 * t25845 - F::cast_from(2.0_f64) * t4700 * t23738 * t4696 - F::cast_from(6.0_f64) * t4700 * t83479 * t1637 * t3213 + F::cast_from(4.0_f64) * t4700 * t23742 * t60941 + F::cast_from(2.0_f64) * t4700 * t23742 * t1637 * t3209 - t4700 * t6822 * t14662;
    t89729
}
