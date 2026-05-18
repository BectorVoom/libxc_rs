//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1418/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1418<F: Float>(t120297: F, t120304: F, t122204: F, t122206: F, t122210: F, t122213: F, t122218: F, t16460: F, t2092: F, t26224: F, t26226: F, t26481: F, t26989: F, t27062: F, t33294: F, t33320: F, t3758: F, t6958: F, t8637: F, t91488: F) -> F {
    let t122223 = -F::new(6.0) * t26224 * t26989 * t26481 + F::new(2.0) * t6958 * t27062 + F::new(0.16449340668482264365e-1) * t122204 - F::new(6.0) * t122206 * t26226 - t3758 * t33294 + t120297 + F::new(0.19190897446562641759e-1) * t122210 + F::new(0.16449340668482264365e-1) * t122213 - t91488 * t2092 - F::new(0.3289868133696452873e-1) * t122218 + t120304 + F::new(2.0) * t3758 * t33320 - t16460 * t8637;
    t122223
}
