//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1274/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1274<F: Float>(t31169: F, t3777: F, t1336: F, t1338: F, t241: F, t835: F, t31172: F, t240: F, t3787: F, t22824: F, t31159: F, t22866: F, t8462: F) -> (F, F, F, F, F, F) {
    let t114002 = t3777 * t31169;
    let t114011 = t1336 * t1338 * t835 * t241;
    let t114012 = t114011 * t31172;
    let t114013 = F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t114012;
    let t114016 = t1336 * t3787 * t240 * t241;
    let t114025 = t22824 * t31159;
    let t114027 = t22866 * t8462;
    (t114002, t114011, t114013, t114016, t114025, t114027)
}
