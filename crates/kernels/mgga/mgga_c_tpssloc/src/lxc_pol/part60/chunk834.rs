//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 834/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk834<F: Float>(t460: F, t6144: F, t7320: F, t6138: F, t1748: F, t2134: F, t24729: F, t24733: F, t24741: F, t27604: F, t27626: F, t27651: F, t6192: F, t6221: F, t6227: F, t6232: F, t7339: F, t8028: F, t8031: F, t8035: F) -> F {
    let t29614 = t6144 * t460;
    let t29615 = t29614 * t7320;
    let t29624 = t6138 * t460;
    let t29625 = t29624 * t7320;
    let t29636 = -t27626 / F::new(432.0) - F::cast_from(0.20186378047070195428e-3_f64) * t27651 - F::cast_from(0.10093189023535097714e-3_f64) * t2134 * t29615 - t24741 * t6192 / F::new(1152.0) + F::cast_from(0.16149102437656156342e-2_f64) * t8028 * t8035 + F::cast_from(0.20186378047070195428e-3_f64) * t8031 * t8035 - F::cast_from(0.10093189023535097714e-3_f64) * t2134 * t29625 + t7339 * t6221 / F::new(1536.0) + t24729 * t6227 / F::new(768.0) - t24733 * t6232 / F::new(1536.0) + t27604 * t1748 / F::new(216.0);
    t29636
}
