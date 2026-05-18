//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 1088/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk1088<F: Float>(t18024: F, t3071: F, t1009: F, t5848: F, t1011: F, t1019: F, t5873: F, t884: F, t10422: F, t5908: F, t3070: F, t1025: F, t10403: F, t10923: F, t10937: F, t14194: F, t14203: F, t14495: F, t14503: F, t18008: F, t18010: F, t18016: F, t18021: F, t3117: F, t378: F, t5900: F, t5909: F) -> (F, F) {
    let t18025 = t3071 * t18024;
    let t18028 = t5848 * t1009;
    let t18029 = t18028 * t1011;
    let t18030 = t18029 * t1019;
    let t18035 = t5873 * t884;
    let t18036 = t3071 * t18035;
    let t18041 = t10422 * t5908;
    let t18042 = t3070 * t18041;
    let t18044 = t18008 / F::new(3456.0) + t14194 - t18010 * t378 / F::new(576.0) - t14203 / F::new(10368.0) + t10403 * t18016 / F::new(1152.0) - t10923 / F::new(1296.0) + t3070 * t18021 / F::new(4608.0) - t3070 * t18025 / F::new(1152.0) + t14495 + t18030 * t1025 / F::new(3072.0) - t3117 * t5900 / F::new(2304.0) + t14503 + t10403 * t18036 / F::new(2304.0) - t10937 * t5909 / F::new(432.0) + t18042 / F::new(3456.0);
    (t18028, t18044)
}
