//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1839/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1839<F: Float>(t10295: F, t10296: F, t10298: F, t10300: F, t10302: F, t13530: F, t13534: F, t13539: F, t13544: F, t13548: F, t13557: F, t13561: F, t13642: F, t13647: F, t13921: F, t13922: F, t13923: F) -> F {
    let t13931 = t10295 + F::new(10.0) / F::new(27.0) * t10296 - t10298 / F::new(27.0) + F::new(2.0) / F::new(9.0) * t10300 - t10302 / F::new(9.0) + F::new(5.0) / F::new(27.0) * t13642 - t13921 + t13922 - t13923 + F::new(2.0) / F::new(27.0) * t13539 - t13557 / F::new(3.0) + t13530 / F::new(9.0) + t13534 / F::new(18.0) + t13561 - F::new(2.0) / F::new(3.0) * t13544 - t13548 / F::new(3.0) + t13647 / F::new(6.0);
    t13931
}
