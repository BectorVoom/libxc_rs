//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1034/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1034<F: Float>(t115508: F, t122187: F, t122192: F, t122204: F, t122210: F, t122213: F, t122218: F, t16460: F, t24082: F, t26996: F, t27068: F, t27132: F, t32151: F, t32176: F, t33798: F, t3882: F, t5215: F, t5321: F, t7194: F, t7199: F, t7925: F, t8801: F) -> F {
    let t124093 = F::cast_from(2.0_f64) * t3882 * t33798 + F::cast_from(2.0_f64) * t5215 * t32176 + F::cast_from(4.0_f64) * t7194 * t27132 + F::cast_from(4.0_f64) * t7194 * t26996 + F::cast_from(0.6579736267392905746e-1_f64) * t122187 - F::cast_from(0.15352717957250113407e0_f64) * t115508 - F::cast_from(0.3289868133696452873e-1_f64) * t122192 + F::cast_from(4.0_f64) * t27068 * t7199 + F::cast_from(0.6579736267392905746e-1_f64) * t122204 + F::cast_from(4.0_f64) * t24082 * t7925 - t5215 * t32151 - t16460 * t8801 + F::cast_from(0.76763589786250567037e-1_f64) * t122210 + F::cast_from(0.6579736267392905746e-1_f64) * t122213 - F::cast_from(0.13159472534785811492e0_f64) * t122218 + F::cast_from(2.0_f64) * t5321 * t32176;
    t124093
}
