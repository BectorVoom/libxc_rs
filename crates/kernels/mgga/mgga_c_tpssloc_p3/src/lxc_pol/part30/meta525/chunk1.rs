//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1866/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1866<F: Float>(t2006: F, t5210: F, t1807: F, t6955: F, t22646: F, t26184: F, t26187: F, t26191: F, t26195: F, t26198: F, t26200: F, t26204: F, t26207: F, t26212: F, t26217: F, t568: F) -> (F, F, F) {
    let t26219 = t5210 * t2006;
    let t26221 = t1807 * t6955;
    let t26223 = F::cast_from(0.38381794893125283518e-1_f64) * t26184 - F::cast_from(0.16449340668482264365e-1_f64) * t26187 - F::cast_from(0.16449340668482264365e-1_f64) * t26191 - F::cast_from(0.16449340668482264365e-1_f64) * t26195 + F::cast_from(0.82246703342411321825e-2_f64) * t26198 + F::cast_from(0.19190897446562641759e-1_f64) * t26200 - F::cast_from(0.82246703342411321825e-2_f64) * t26204 - F::cast_from(0.82246703342411321825e-2_f64) * t26207 + F::cast_from(0.82246703342411321825e-2_f64) * t26212 + F::cast_from(0.16449340668482264365e-1_f64) * t26217 - t22646 + t26219 * t568 + t26221 * t568;
    (t26219, t26221, t26223)
}
