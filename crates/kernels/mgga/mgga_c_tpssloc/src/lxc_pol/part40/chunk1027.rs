//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1027/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1027<F: Float>(t5873: F, t884: F, t3071: F, t10422: F, t5908: F, t3070: F, t1025: F, t10403: F, t10923: F, t10937: F, t14194: F, t14203: F, t14495: F, t14503: F, t18008: F, t18010: F, t18016: F, t18021: F, t18025: F, t18030: F, t3117: F, t378: F, t5900: F, t5909: F) -> (F,) {
    let t18035 = t5873 * t884;
    let t18036 = t3071 * t18035;
    let t18041 = t10422 * t5908;
    let t18042 = t3070 * t18041;
    let t18044 = t18008 / 3456.0 + t14194 - t18010 * t378 / 576.0 - t14203 / 10368.0 + t10403 * t18016 / 1152.0 - t10923 / 1296.0 + t3070 * t18021 / 4608.0 - t3070 * t18025 / 1152.0 + t14495 + t18030 * t1025 / 3072.0 - t3117 * t5900 / 2304.0 + t14503 + t10403 * t18036 / 2304.0 - t10937 * t5909 / 432.0 + t18042 / 3456.0;
    (t18044,)
}
