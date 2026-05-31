//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2322/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2322<F: Float>(t1193: F, t29585: F, t2136: F, t29562: F, t52: F, t27674: F, t5040: F, t1409: F, t8027: F, t18368: F, t27629: F, t27692: F, t4954: F, t8040: F, t86324: F, t95384: F, t95404: F, t95410: F, t95424: F, t95435: F, t95566: F, t95678: F) -> F {
    let t104139 = t29585 * t1193;
    let t104142 = t29562 * t52 * t2136;
    let t104150 = t27674 * t5040;
    let t104153 = t8027 * t1409 * t2136;
    let t104155 = t95566 * t4954 / F::cast_from(216.0_f64) - t86324 * t18368 / F::cast_from(1152.0_f64) + t95404 + F::cast_from(11.0_f64) / F::cast_from(324.0_f64) * t104139 + F::cast_from(0.72670960969452703541e-2_f64) * t104142 - F::cast_from(0.20186378047070195428e-3_f64) * t95678 * t8040 - F::cast_from(0.20186378047070195428e-3_f64) * t27629 * t27692 + F::cast_from(0.20186378047070195428e-3_f64) * t95384 * t8040 + t104150 / F::cast_from(162.0_f64) - t95410 - t95424 - t95435 + F::cast_from(0.16149102437656156342e-2_f64) * t104153;
    t104155
}
