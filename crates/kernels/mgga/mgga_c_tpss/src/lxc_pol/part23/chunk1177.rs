//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1177/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1177<F: Float>(t5965: F, t7682: F, t7690: F, t18351: F, t1860: F, t1675: F, t18305: F, t18338: F, t18347: F, t18350: F, t18356: F, t18360: F, t18363: F, t18366: F, t1861: F, t19192: F, t19220: F, t19223: F, t19226: F, t5483: F, t5489: F, t5492: F, t5966: F, t5976: F, t5979: F) -> (F, F, F, F) {
    let t19229 = t7682 * t5965;
    let t19232 = t7690 * t5965;
    let t19235 = t1860 * t18351;
    let t19238 = 2.0 / 3.0 * t18338 * t1861 + 5.0 / 3.0 * t19192 * t5489 + 5.0 / 3.0 * t5966 * t18356 + 5.0 / 6.0 * t5966 * t18360 + t18363 * t1861 / 3.0 + t18366 * t1861 / 3.0 + 2.0 / 3.0 * t5492 * t5976 + 2.0 / 3.0 * t5492 * t5979 - t18305 * t1861 / 6.0 - t5483 * t5976 / 3.0 - t5483 * t5979 / 3.0 - t1675 * t19220 / 6.0 - t1675 * t19223 / 3.0 - t1675 * t19226 / 6.0 + 5.0 / 3.0 * t19229 * t5489 - 5.0 * t19232 * t18347 - 10.0 / 3.0 * t18350 * t19235;
    (t19229, t19232, t19235, t19238)
}
