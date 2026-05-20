//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2449/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2449<F: Float>(t10250: F, t1041: F, t10884: F, t14172: F, t14184: F, t1607: F, t1616: F, t1618: F, t3048: F, t3070: F, t3071: F, t3117: F, t42358: F, t42554: F, t42756: F, t43167: F, t4582: F, t4593: F, t48554: F, t50078: F, t50084: F, t50094: F, t50098: F, t50100: F) -> F {
    let t50102 = F::new(5.0) / F::new(4608.0) * t3117 * t14184 - F::new(5.0) / F::new(768.0) * t1041 * t4582 * t14172 * t48554 + t42756 * t1618 / F::new(3072.0) + t50078 - t3070 * t3071 * t1616 * t10250 / F::new(768.0) - t50084 / F::new(1152.0) + t43167 / F::new(768.0) - t42358 * t4582 * t4593 * t10884 / F::new(3072.0) - F::new(5.0) / F::new(864.0) * t3048 * t14184 + t50094 / F::new(1152.0) - F::new(77.0) / F::new(486.0) * t42554 * t1607 + F::new(11.0) / F::new(324.0) * t50098 + t50100 / F::new(144.0);
    t50102
}
