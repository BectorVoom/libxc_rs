//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 912/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk912<F: Float>(t33822: F, t539: F, t1375: F, t32127: F, t32154: F, t33241: F, t33247: F, t33251: F, t33274: F, t33298: F, t33798: F, t33804: F, t33810: F, t33815: F, t568: F, t7194: F, t7925: F) -> (F, F) {
    let t33823 = t539 * t33822;
    let t33825 = -F::cast_from(0.3289868133696452873e-1_f64) * t33241 + F::cast_from(2.0_f64) * t1375 * t33798 + F::cast_from(0.3289868133696452873e-1_f64) * t33247 + F::cast_from(0.6579736267392905746e-1_f64) * t33251 - t32127 + t32154 + F::cast_from(4.0_f64) * t1375 * t33804 + F::cast_from(0.6579736267392905746e-1_f64) * t33274 - F::cast_from(0.3289868133696452873e-1_f64) * t33298 - F::cast_from(6.0_f64) * t1375 * t33810 + F::cast_from(4.0_f64) * t7194 * t7925 + t33815 * t568 + t33823 * t568;
    (t33823, t33825)
}
