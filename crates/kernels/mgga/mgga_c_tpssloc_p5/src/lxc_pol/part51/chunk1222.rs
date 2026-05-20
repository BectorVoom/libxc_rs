//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1222/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1222<F: Float>(t1375: F, t1843: F, t2016: F, t2092: F, t26366: F, t26477: F, t27009: F, t31129: F, t31571: F, t31653: F, t32696: F, t32764: F, t32771: F, t33241: F, t33247: F, t33251: F, t33269: F, t33274: F, t33294: F, t33298: F, t33301: F, t33332: F, t5215: F, t5321: F, t6958: F, t7194: F, t7750: F, t7937: F, t8637: F) -> F {
    let t33334 = -F::cast_from(0.82246703342411321825e-2_f64) * t33241 - t31653 * t1843 + F::cast_from(0.82246703342411321825e-2_f64) * t33247 + F::cast_from(0.16449340668482264365e-1_f64) * t33251 - t7194 * t7750 + t32696 - t5215 * t8637 - t31571 - t26366 * t2092 - t26477 * t2092 + t33269 + t31129 + t32764 - t27009 * t2016 + F::cast_from(0.16449340668482264365e-1_f64) * t33274 - t1375 * t33294 - F::cast_from(0.82246703342411321825e-2_f64) * t33298 - t32771 + F::new(2.0) * t1375 * t33301 - t5321 * t8637 - t6958 * t7937 + t33332;
    t33334
}
