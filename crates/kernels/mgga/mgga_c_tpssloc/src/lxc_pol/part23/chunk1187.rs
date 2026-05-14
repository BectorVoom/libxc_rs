//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1187/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1187<F: Float>(t40: F, t52: F, t16549: F, t20217: F, t2433: F, t40632: F, t4080: F, t5398: F, t73: F, t75836: F, t75847: F, t75912: F, t16563: F, t2440: F, t40647: F, t4087: F, t76: F, t157: F, t182: F, zeta_threshold: F) -> (F, F) {
    let t146 = t40 <= zeta_threshold;
    let t150 = t52 <= zeta_threshold;
    let t75916 = piecewise3(t146, 0.0, 40.0 / 81.0 * t40632 * t75836 - 16.0 / 9.0 * t16549 * t5398 + 4.0 / 3.0 * t2433 * t75847 + 16.0 / 9.0 * t4080 * t20217 + 4.0 / 3.0 * t73 * t75912);
    let t75928 = piecewise3(t150, 0.0, 40.0 / 81.0 * t40647 * t75836 + 16.0 / 9.0 * t16563 * t5398 + 4.0 / 3.0 * t2440 * t75847 + 16.0 / 9.0 * t4087 * t20217 - 4.0 / 3.0 * t76 * t75912);
    let t75929 = t75916 + t75928;
    let t75932 = 0.19751673498613801407e-1 * t75929 * t157 * t182;
    (t75929, t75932)
}
