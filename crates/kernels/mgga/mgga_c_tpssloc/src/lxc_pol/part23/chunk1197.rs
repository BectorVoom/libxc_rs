//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1197/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1197<F: Float>(t5527: F, t5544: F, t1504: F, t1506: F, t16729: F, t16736: F, t20800: F, t20835: F, t20843: F, t20846: F, t20849: F, t225: F, t228: F, t230: F, t2671: F, t41315: F, t4225: F, t4226: F, t5601: F, t5605: F, t5608: F, t75978: F, t76006: F, t76007: F, t76009: F, t76010: F, t76013: F, t76014: F, t76021: F, t76038: F, t824: F) -> (F, F, F) {
    let t76056 = t5527 * t5527;
    let t76063 = t5544 * t5544;
    let t76073 = -(t76006 + t76007 + t76009 + t76010 + t76013 + t76014 + t76021 + t76038) * t225 * t230 + 12.0 * t20835 * t1506 - 72.0 * t5601 * t5605 + 18.0 * t5601 * t5608 + 240.0 * t1504 * t20843 - 144.0 * t16729 * t20846 + 12.0 * t1504 * t20849 - 360.0 * t228 * t41315 * t76056 + 360.0 * t4225 * t16736 * t5544 - 36.0 * t228 * t2671 * t76063 - 48.0 * t4225 * t4226 * t20800 + 3.0 * t228 * t824 * t75978;
    (t76056, t76063, t76073)
}
