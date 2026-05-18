//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 1030/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk1030<F: Float>(t115919: F, t115920: F, t115922: F, t115924: F, t115927: F, t115929: F, t115942: F, t115946: F, t115948: F, t115959: F, t115965: F, t115968: F, t117531: F, t1266: F, t2114: F, t23918: F, t23938: F, t24428: F, t24932: F, t32349: F, t510: F, t7061: F, t7266: F, t7271: F) -> F {
    let t117659 = -t117531 * t510 - F::new(2.0) * t1266 * t32349 - t2114 * t24428 - F::new(2.0) * t23918 * t7266 - F::new(4.0) * t23938 * t7271 - F::new(4.0) * t24932 * t7061 - t115919 - t115920 + t115922 + t115924 - t115927 - t115929 - t115942 - t115946 - t115948 + t115959 + t115965 - t115968;
    t117659
}
