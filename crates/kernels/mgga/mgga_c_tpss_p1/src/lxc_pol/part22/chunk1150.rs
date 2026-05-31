//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1150/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1150<F: Float>(t12836: F, t3348: F, t774: F, t1625: F, t3234: F, t10077: F, t1642: F, t3245: F, t9986: F, t1244: F, t12819: F, t12825: F, t12831: F, t12835: F, t3271: F, t4413: F, t9981: F, t9991: F, t9995: F, t9997: F) -> (F, F, F, F, F, F) {
    let t12838 = t3348 * t774 * t12836;
    let t12841 = t1625 * t3234;
    let t12843 = t3348 * t774 * t12841;
    let t12846 = t10077 * t1642;
    let t12851 = t1625 * t3245;
    let t12853 = t9986 * t774 * t12851;
    let t12856 = F::cast_from(7.0_f64) / F::cast_from(4608.0_f64) * t9981 - F::cast_from(5.0_f64) / F::cast_from(384.0_f64) * t3271 * t12819 + t3271 * t12825 / F::cast_from(384.0_f64) - t4413 * t12831 / F::cast_from(192.0_f64) - t12835 + F::cast_from(5.0_f64) / F::cast_from(384.0_f64) * t1244 * t12838 + F::cast_from(5.0_f64) / F::cast_from(768.0_f64) * t1244 * t12843 - F::cast_from(119.0_f64) / F::cast_from(13824.0_f64) * t12846 - F::cast_from(35.0_f64) / F::cast_from(1152.0_f64) * t9991 - F::cast_from(119.0_f64) / F::cast_from(1728.0_f64) * t9995 + F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t9997 - F::cast_from(5.0_f64) / F::cast_from(128.0_f64) * t1244 * t12853;
    (t12838, t12841, t12843, t12851, t12853, t12856)
}
