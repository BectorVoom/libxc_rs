//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 456/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk456<F: Float>(t3142: F, t974: F, t2770: F, t2978: F, t2244: F, t2775: F, t976: F, t1005: F, t1036: F, t221: F, t2965: F, t339: F) -> (F, F, F, F, F, F) {
    let t3143 = t974 * t3142;
    let t3146 = t2978 * t2770;
    let t3147 = t3146 * t2244;
    let t3148 = t974 * t3147;
    let t3151 = t976 * t2775;
    let t3152 = t3151 * t2244;
    let t3153 = t974 * t3152;
    let t3156 = t1005 * t1036;
    let t3158 = t221 * t2965;
    let t3160 = t339 * t3158 / F::cast_from(432.0_f64);
    (t3143, t3148, t3153, t3156, t3158, t3160)
}
