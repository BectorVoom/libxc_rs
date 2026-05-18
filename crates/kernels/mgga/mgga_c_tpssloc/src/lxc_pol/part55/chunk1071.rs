//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1071/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1071<F: Float>(t1222: F, t8879: F, t477: F, t483: F, t372: F, t471: F) -> (F, F, F, F) {
    let t32445 = t8879 * t1222 / F::new(2304.0);
    let t32446 = t477 * t483;
    let t32447 = t32446 * t372;
    let t32448 = t471 * t32447;
    (t32445, t32446, t32447, t32448)
}
