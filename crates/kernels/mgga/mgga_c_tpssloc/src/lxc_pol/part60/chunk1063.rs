//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1063/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1063<F: Float>(t124673: F, t125050: F, t125053: F, t125065: F, t125067: F, t125068: F, t125069: F, t125071: F, t130275: F, t130495: F, t130498: F, t1398: F, t1852: F, t1858: F, t2099: F, t2105: F, t2170: F, t2174: F, t29396: F, t29430: F, t29866: F, t29884: F, t3: F, t34176: F, t34194: F, t580: F, t6471: F, t6483: F, t7946: F, t7961: F, t8111: F, t8119: F, t8844: F, t8852: F) -> F {
    let tv4rho2sigma216 = t6471 * t8852 + F::new(2.0) * t7946 * t8119 + F::new(2.0) * t124673 + t1398 * (t130275 + t130498) + F::new(2.0) * t125069 + t29866 * t2105 + t3 * t130495 * t580 + t2099 * t29884 + F::new(2.0) * t125065 + F::new(2.0) * t125067 + t2170 * t29430 + F::new(2.0) * t125071 + F::new(2.0) * t8111 * t7961 + t29396 * t2174 + F::new(2.0) * t125053 + F::new(2.0) * t125050 + F::new(2.0) * t125068 + F::new(2.0) * t34176 * t1858 + F::new(2.0) * t1852 * t34194 + t8844 * t6483;
    tv4rho2sigma216
}
