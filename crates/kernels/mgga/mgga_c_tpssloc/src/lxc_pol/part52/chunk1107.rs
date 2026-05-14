//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1107/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1107<F: Float>(t1874: F, t33690: F, t7266: F, t7461: F, t27863: F, t1459: F, t1774: F, t1869: F, t1976: F, t2114: F, t2165: F, t31880: F, t32659: F, t33686: F, t33688: F, t510: F, t6517: F, t7451: F, t7670: F, t7983: F, t7989: F, t8103: F, t8667: F) -> (F,) {
    let t33691 = t33690 * t1874;
    let t33693 = t7266 * t7461;
    let t33697 = t27863 * t1874;
    let t33702 = -2.0 * t1459 * t31880 - t1774 * t8667 - t1869 * t8103 - t1976 * t7983 - t2114 * t7670 - t2165 * t7451 - t33686 * t510 - 2.0 * t6517 * t7989 - 2.0 * t32659 - 2.0 * t33688 - 2.0 * t33691 - 2.0 * t33693 - 2.0 * t33697;
    (t33702,)
}
