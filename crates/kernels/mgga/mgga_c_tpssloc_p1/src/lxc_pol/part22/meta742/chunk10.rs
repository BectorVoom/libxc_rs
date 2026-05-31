//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2462/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2462<F: Float>(t4657: F, t5872: F, t1022: F, t1058: F, t1060: F, t1063: F, t11034: F, t11059: F, t11060: F, t1610: F, t18086: F, t18089: F, t18094: F, t18129: F, t21481: F, t21614: F, t21626: F, t21637: F, t21657: F, t3186: F, t3200: F, t3201: F, t43553: F, t43554: F, t4649: F, t4669: F, t4673: F, t4678: F, t47853: F, t5928: F) -> (F, F) {
    let t69996 = t4657 * t5872;
    let t70009 = t1022 * t1058 * t1060 * t21614 - F::cast_from(36.0_f64) * t1022 * t21637 * t43553 * t43554 + F::cast_from(18.0_f64) * t11059 * t11060 * t4649 * t5928 + F::cast_from(6.0_f64) * t21626 * t3186 * t4673 - F::cast_from(3.0_f64) * t3200 * t3201 * t69996 + t1063 * t21481 + F::cast_from(6.0_f64) * t11034 * t21657 + F::cast_from(3.0_f64) * t1610 * t18129 + F::cast_from(3.0_f64) * t18086 * t4678 + F::cast_from(6.0_f64) * t18089 * t4669 + F::cast_from(3.0_f64) * t18094 * t47853;
    (t69996, t70009)
}
