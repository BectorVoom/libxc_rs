//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2032/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2032<F: Float>(t1375: F, t1385: F, t16439: F, t1843: F, t20023: F, t20044: F, t26996: F, t27062: F, t29360: F, t3887: F, t5321: F, t6460: F, t7194: F, t7213: F, t7214: F, t7925: F, t93341: F, t97640: F, t97644: F, t97647: F) -> F {
    let t102900 = F::new(4.0) * t5321 * t27062 + F::new(4.0) * t16439 * t7925 - t20044 * t7214 + F::new(2.0) * t1375 * t3887 * t7213 * t6460 - t7194 * t20023 + F::new(4.0) * t5321 * t26996 + F::new(2.0) * t1375 * t3887 * t29360 * t1385 - F::new(2.0) * t93341 * t1843 + F::cast_from(0.3289868133696452873e-1_f64) * t97640 + F::cast_from(0.6579736267392905746e-1_f64) * t97644 + F::cast_from(0.6579736267392905746e-1_f64) * t97647;
    t102900
}
