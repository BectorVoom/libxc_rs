//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1275/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1275<F: Float>(t1336: F, t1338: F, t241: F, t835: F, t31172: F, t240: F, t3787: F, t22824: F, t31159: F, t22866: F, t8462: F, t1307: F, t22690: F, t22792: F, t6950: F) -> (F, F, F, F, F, F) {
    let t114011 = t1336 * t1338 * t835 * t241;
    let t114012 = t114011 * t31172;
    let t114013 = F::new(7.0) / F::new(1152.0) * t114012;
    let t114016 = t1336 * t3787 * t240 * t241;
    let t114025 = t22824 * t31159;
    let t114027 = t22866 * t8462;
    let t114031 = t22792 * t22690 * t6950 * t1307;
    (t114011, t114013, t114016, t114025, t114027, t114031)
}
