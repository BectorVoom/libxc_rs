//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1276/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1276<F: Float>(t32441: F, t4997: F, t1017: F, t1207: F, t1209: F, t1742: F, t372: F, t471: F, t477: F, t32440: F, t5001: F, t1730: F, t32447: F) -> (F, F, F, F, F) {
    let t125398 = t32441 * t4997;
    let t125402 = t1207 * t1209 * t1742 * t1017;
    let t125407 = t471 * t477 * t1742 * t372;
    let t125410 = t5001 * t32440;
    let t125413 = t1730 * t32447;
    (t125398, t125402, t125407, t125410, t125413)
}
