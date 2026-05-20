//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2337/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2337<F: Float>(t24574: F, t29694: F, t1170: F, t2121: F, t29670: F, t29678: F, t7280: F, t14972: F, t15820: F, t1761: F, t18571: F, t2144: F, t24893: F, t27383: F, t27396: F, t27406: F, t27427: F, t29795: F, t3487: F, t4945: F, t498: F, t6150: F, t6268: F, t7348: F, t8088: F, t86451: F, t94759: F, t95899: F) -> F {
    let t104509 = t24574 * t29694;
    let t104521 = t2121 * t1170 * t29670;
    let t104527 = t29678 * t7280;
    let t104534 = -F::cast_from(0.18277045187202515961e-2_f64) * t104509 - F::cast_from(0.43864908449286038306e-1_f64) * t27406 * t27383 + F::cast_from(0.14621636149762012769e-1_f64) * t27406 * t27427 + F::new(4.0) * t4945 * t27396 - t3487 * t29795 - F::new(2.0) * t15820 * t8088 + F::cast_from(0.27415567780803773942e-2_f64) * t104521 + t18571 * t2144 * t498 + t6150 * t7348 * t498 + F::cast_from(0.26806332941230356743e-1_f64) * t104527 - F::new(2.0) * t95899 * t1761 - t24893 * t6268 - F::new(2.0) * t14972 * t8088 - t94759 + t86451;
    t104534
}
