//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 983/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk983<F: Float>(t13087: F, t13182: F, t13234: F, t16848: F, t16877: F, t16879: F, t20882: F, t20887: F, t20891: F, t20896: F, t20958: F, t20998: F, t21011: F, t2643: F, t843: F) -> F {
    let t21013 = -F::new(35.0) / F::new(72.0) * t13087 - F::new(119.0) / F::new(4608.0) * t13182 + t2643 * t20882 / F::new(256.0) + t2643 * t20887 / F::new(256.0) - t2643 * t20891 / F::new(1024.0) - F::new(7.0) / F::new(192.0) * t16848 - F::new(5.0) / F::new(128.0) * t843 * t20896 + F::new(119.0) / F::new(4608.0) * t13234 + F::new(7.0) / F::new(768.0) * t16877 - F::new(7.0) / F::new(768.0) * t16879 + t20958 + t20998 + t21011;
    t21013
}
