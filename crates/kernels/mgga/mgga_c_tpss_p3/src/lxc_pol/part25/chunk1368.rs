//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1368/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1368<F: Float>(t1364: F, t14029: F, t14245: F, t1692: F, t1812: F, t18728: F, t18812: F, t19818: F, t20510: F, t21659: F, t2439: F, t3552: F, t4701: F, t4806: F, t51780: F, t52613: F, t5849: F, t5853: F, t62807: F, t62829: F, t66299: F, t69810: F, t69847: F, t69863: F, t69881: F, t70240: F, t70243: F, t70759: F, t750: F) -> F {
    let t72411 = F::cast_from(6.0_f64) * t1364 * t20510 * t2439 + F::cast_from(3.0_f64) * t14029 * t1812 * t2439 + F::cast_from(12.0_f64) * t14245 * t1812 * t3552 + F::cast_from(4.0_f64) * t1692 * t18812 * t69881 + F::cast_from(2.0_f64) * t1692 * t18812 * t70240 + F::cast_from(4.0_f64) * t1692 * t19818 * t66299 + F::cast_from(2.0_f64) * t1692 * t4806 * t62829 - F::cast_from(6.0_f64) * t1692 * t62807 * t70243 + F::cast_from(6.0_f64) * t18812 * t2439 * t69847 + F::cast_from(3.0_f64) * t21659 * t2439 * t750 + F::cast_from(3.0_f64) * t2439 * t4701 * t5849 - F::cast_from(3.0_f64) * t2439 * t52613 * t5853 - F::cast_from(6.0_f64) * t2439 * t5853 * t69810 - F::cast_from(3.0_f64) * t2439 * t5853 * t69863 - F::cast_from(6.0_f64) * t3552 * t51780 * t5853 + F::cast_from(12.0_f64) * t18728 * t70759;
    t72411
}
