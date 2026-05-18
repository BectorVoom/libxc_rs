//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1094/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1094<F: Float>(t2267: F, t38: F, t240: F, t2244: F, t2250: F, t22502: F, t2261: F, t44: F, t607: F, t6500: F, t67: F, t1864: F) -> (F, F, F, F) {
    let t22505 = t38 * t2267;
    let t22510 = F::new(88.0) / F::new(9.0) * t240;
    let t22511 = F::new(88.0) / F::new(9.0) * t2261 * t44 - F::new(40.0) / F::new(9.0) * t22502 * t607 + F::new(5.0) / F::new(18.0) * t22505 * t2244 + F::new(5.0) / F::new(6.0) * t6500 * t2250 - t22510;
    let t22512 = t22511 * t67;
    let t22513 = t22512 * t1864;
    (t22505, t22511, t22512, t22513)
}
