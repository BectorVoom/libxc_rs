//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1425/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1425<F: Float>(t3886: F, t7936: F, t1385: F, t1992: F, t22635: F, t31559: F, t90566: F, t33246: F, t6883: F, t115339: F, t115341: F, t120218: F, t120221: F, t120226: F, t120229: F, t2092: F, t26477: F, t27115: F, t31642: F, t5215: F, t6958: F, t7214: F, t91491: F) -> F {
    let t122142 = t3886 * t7936;
    let t122145 = t1992 * t22635 * t122142 * t1385;
    let t122150 = t1992 * t90566 * t31559;
    let t122152 = t6883 * t33246;
    let t122155 = -t6958 * t27115 + F::new(0.38381794893125283518e-1) * t115339 + F::new(0.19190897446562641759e-1) * t115341 + F::new(0.16449340668482264365e-1) * t122145 - t5215 * t31642 - t91491 * t2092 - t120218 - t120221 + F::new(0.16449340668482264365e-1) * t122150 - F::new(0.19190897446562641759e-1) * t122152 + t120226 - t26477 * t7214 + t120229;
    t122155
}
