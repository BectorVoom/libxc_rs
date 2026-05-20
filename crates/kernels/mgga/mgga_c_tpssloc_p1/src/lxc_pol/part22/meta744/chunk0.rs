//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2468/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2468<F: Float>(t17611: F, t4641: F, t10480: F, t21391: F, t248: F, t3101: F, t1041: F, t10457: F, t21118: F, t1616: F, t607: F, t10403: F, t10408: F, t10413: F, t1618: F, t17151: F, t17177: F, t17182: F, t17923: F, t3070: F, t3071: F, t42397: F, t42483: F, t5685: F, t61744: F, t61754: F, t61768: F, t61782: F, t62850: F, t70082: F, t70086: F) -> (F, F, F, F) {
    let t70214 = t4641 * t17611;
    let t70227 = t10480 * t248 * t3101 * t21391;
    let t70239 = t1041 * t248 * t10457 * t21118;
    let t70241 = t1616 * t607;
    let t70268 = F::new(5.0) / F::new(6912.0) * t61744 - t61754 * t1618 / F::new(192.0) - F::new(5.0) / F::new(3456.0) * t70239 - t3070 * t3071 * t17182 * t70241 / F::new(768.0) + t10403 * t3071 * t5685 * t70082 / F::new(768.0) + F::new(5.0) / F::new(4608.0) * t3070 * t10408 * t17177 * t70241 - t10413 * t3071 * t5685 * t70086 / F::new(1536.0) + t42483 * t3071 * t62850 * t17923 / F::new(1536.0) + F::new(5.0) / F::new(3456.0) * t61768 + F::new(5.0) / F::new(1728.0) * t3070 * t42397 * t17151 * t70241 - t61782 / F::new(6912.0);
    (t70214, t70227, t70241, t70268)
}
