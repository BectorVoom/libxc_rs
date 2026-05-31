//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1367/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1367<F: Float>(t1398: F, t14076: F, t14256: F, t14426: F, t1692: F, t1812: F, t18807: F, t198: F, t19809: F, t20417: F, t20514: F, t207: F, t21262: F, t21678: F, t2439: F, t3552: F, t3610: F, t36547: F, t3683: F, t3724: F, t4706: F, t4802: F, t52639: F, t5849: F, t5853: F, t6354: F, t66281: F, t70771: F, t72172: F, t72265: F, t821: F, t823: F) -> F {
    let t72363 = -t1692 * t18807 * t4802 - F::cast_from(2.0_f64) * t1692 * t66281 * t1398 + F::cast_from(6.0_f64) * t3552 * t5849 * t4706 + F::cast_from(6.0_f64) * t36547 * t21678 - F::cast_from(2.0_f64) * t1692 * t20514 * t3724 + F::cast_from(6.0_f64) * t2439 * t6354 * t3610 + F::cast_from(12.0_f64) * t3552 * t6354 * t3683 - F::cast_from(12.0_f64) * t20417 * t70771 - t1692 * t72265 * t821 + t198 * t207 * t72172 * t823 - t1692 * t5853 * t14426 - F::cast_from(6.0_f64) * t2439 * t5853 * t52639 - F::cast_from(6.0_f64) * t2439 * t18807 * t21262 - F::cast_from(6.0_f64) * t2439 * t20514 * t14076 + F::cast_from(6.0_f64) * t3552 * t1812 * t14256 - F::cast_from(6.0_f64) * t2439 * t20514 * t19809;
    t72363
}
