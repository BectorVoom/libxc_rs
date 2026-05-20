//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2173/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2173<F: Float>(t12020: F, t1842: F, t1307: F, t193: F, t111: F, t5363: F, t6470: F, t19530: F, t626: F, t1447: F, t2349: F, t2281: F, t5489: F) -> (F, F, F, F, F, F, F) {
    let t55118 = t12020 * t1842;
    let t55224 = t193 * t1307;
    let t55353 = t5363 * t111;
    let t55388 = t6470 * t111;
    let t55420 = t626 * t19530;
    let t55491 = t1447 * t2349;
    let t55531 = t2281 * t5489;
    (t55118, t55224, t55353, t55388, t55420, t55491, t55531)
}
