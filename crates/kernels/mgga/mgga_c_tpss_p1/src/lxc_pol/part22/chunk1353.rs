//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1353/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1353<F: Float>(t1364: F, t1398: F, t14076: F, t1692: F, t18803: F, t18807: F, t18812: F, t19809: F, t20417: F, t20510: F, t2116: F, t2439: F, t3552: F, t35530: F, t3610: F, t3724: F, t44170: F, t44329: F, t44350: F, t44470: F, t44474: F, t5849: F, t5853: F, t62820: F, t6354: F, t6365: F, t63863: F, t66281: F, t750: F, t821: F) -> F {
    let t66704 = F::cast_from(3.0_f64) * t1364 * t18803 * t2439 - t1398 * t1692 * t62820 - F::cast_from(6.0_f64) * t14076 * t18807 * t2439 - F::cast_from(2.0_f64) * t1692 * t18807 * t3724 + F::cast_from(4.0_f64) * t1692 * t18812 * t44350 - F::cast_from(2.0_f64) * t1692 * t66281 * t821 - F::cast_from(6.0_f64) * t18807 * t19809 * t2439 + F::cast_from(6.0_f64) * t18812 * t2439 * t44474 + F::cast_from(6.0_f64) * t20510 * t2439 * t750 + F::cast_from(6.0_f64) * t2116 * t3552 * t6354 + F::cast_from(6.0_f64) * t2439 * t3610 * t5849 - F::cast_from(6.0_f64) * t2439 * t44329 * t5853 - F::cast_from(6.0_f64) * t2439 * t44470 * t5853 - F::cast_from(6.0_f64) * t3552 * t5853 * t63863 - F::cast_from(12.0_f64) * t20417 * t44170 + F::cast_from(6.0_f64) * t35530 * t6365;
    t66704
}
