//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 753/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk753<F: Float>(t1877: F, t25: F, t8366: F, t8370: F, t202: F, t8365: F, t8369: F, t193: F, t2752: F, t870: F, t28: F, t8319: F, t88: F, t1268: F, t8326: F, t2006: F, t225: F, t567: F) -> (F, F, F, F, F, F, F, F) {
    let t8374 = t1877 * t8366 * t25 / 2.0 - t1877 * t8370 * t25 / 2.0;
    let t8418 = t202 * t8365;
    let t8421 = t202 * t8369;
    let t8424 = -t193 * t2752 * t8421 + t193 * t8418 * t870;
    let t8434 = t1877 * t8366 * t28 / 2.0 - t1877 * t8370 * t28 / 2.0;
    let t8444 = 2.0 * t88 * t8319;
    let t8445 = t1268 * t8326;
    let t8446 = 2.0 * t8445;
    let t8454 = t2006 * t225 * t567;
    (t8374, t8418, t8421, t8424, t8434, t8444, t8446, t8454)
}
