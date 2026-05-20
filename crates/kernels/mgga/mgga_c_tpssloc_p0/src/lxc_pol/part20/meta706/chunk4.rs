//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2694/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2694<F: Float>(t16028: F, t225: F, t12022: F, t12437: F, t12438: F, t1375: F, t1386: F, t16437: F, t16460: F, t16471: F, t16475: F, t1842: F, t1843: F, t3758: F, t3887: F, t3912: F, t39913: F, t39916: F, t39919: F, t40591: F, t5215: F, t53866: F, t539: F, t54817: F, t568: F) -> F {
    let t54825 = t16028 * t225;
    let t54832 = F::new(24.0) * t12022 * t1375 * t1842 * t40591 + F::new(2.0) * t12437 * t1375 * t1842 * t3887 + t539 * t54817 * t568 - t12438 * t5215 - F::new(6.0) * t1386 * t53866 - F::new(3.0) * t1386 * t54825 - F::new(3.0) * t16437 * t3758 - F::new(3.0) * t16460 * t3912 + F::new(6.0) * t16471 * t3758 - F::new(18.0) * t16475 * t3758 - F::new(3.0) * t1843 * t39913 - F::new(3.0) * t1843 * t39916 - t1843 * t39919;
    t54832
}
