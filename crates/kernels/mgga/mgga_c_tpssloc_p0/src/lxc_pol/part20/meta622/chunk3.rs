//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2241/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2241<F: Float>(t52: F, t10913: F, t12606: F, t12961: F, t1431: F, t2244: F, t2250: F, t4012: F, t4015: F, t4111: F, t45872: F, t607: F, t771: F, t78: F, t9258: F, t9288: F, zeta_threshold: F) -> F {
    let t150 = t52 <= zeta_threshold;
    let t46424 = piecewise3::<F>(t150, F::new(0.0), -F::new(56.0) / F::new(81.0) * t4012 * t9288 - F::new(8.0) / F::new(9.0) * t4015 * t2244 - F::new(8.0) / F::new(9.0) * t1431 * t10913 - F::new(2.0) / F::new(3.0) * t78 * t12606 * t607 - F::new(2.0) / F::new(3.0) * t12961 * t2250 - F::new(2.0) / F::new(9.0) * t4111 * t9258 - F::new(2.0) / F::new(3.0) * t771 * t45872);
    t46424
}
