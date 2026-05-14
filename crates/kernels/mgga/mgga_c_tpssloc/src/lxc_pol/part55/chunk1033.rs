//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1033/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1033<F: Float>(t114011: F, t31172: F, t1336: F, t240: F, t241: F, t3787: F, t22824: F, t31159: F, t22866: F, t8462: F, t1307: F, t22690: F, t22792: F, t6950: F, t1332: F, t31175: F, t8467: F) -> (F, F, F, F, F, F) {
    let t114012 = t114011 * t31172;
    let t114016 = t1336 * t3787 * t240 * t241;
    let t114025 = t22824 * t31159;
    let t114026 = 0.21083550404717759669e-2 * t114025;
    let t114027 = t22866 * t8462;
    let t114028 = 0.45217486825437237757e-1 * t114027;
    let t114031 = t22792 * t22690 * t6950 * t1307;
    let t114034 = t1332 * t31175 * t8467;
    (t114012, t114016, t114026, t114028, t114031, t114034)
}
