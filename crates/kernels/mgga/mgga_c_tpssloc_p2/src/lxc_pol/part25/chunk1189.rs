//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1189/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1189<F: Float>(t80743: F, t12022: F, t12027: F, t12033: F, t12444: F, t1375: F, t2091: F, t24082: F, t3889: F, t40591: F, t7194: F, t7199: F, t7214: F, t80722: F, t80725: F, t80728: F, t80735: F, t80738: F) -> F {
    let t84400 = F::cast_from(0.3244175520728446583e0_f64) * t80743;
    let t84409 = -F::cast_from(6.0_f64) * t12444 * t7214 + F::cast_from(0.38381794893125283518e0_f64) * t80722 + F::cast_from(0.24674011002723396548e-1_f64) * t80725 - F::cast_from(0.69087230807625510332e0_f64) * t80728 - F::cast_from(0.39478417604357434476e0_f64) * t80735 - F::cast_from(0.24674011002723396548e-1_f64) * t80738 + F::cast_from(6.0_f64) * t7194 * t12027 - t84400 + F::cast_from(6.0_f64) * t24082 * t3889 + F::cast_from(24.0_f64) * t1375 * t40591 * t2091 * t12022 + F::cast_from(6.0_f64) * t12033 * t7199;
    t84409
}
