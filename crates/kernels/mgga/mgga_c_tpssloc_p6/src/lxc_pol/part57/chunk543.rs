//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 543/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk543<F: Float>(t344: F, t976: F, t381: F, t225: F, t387: F, t340: F, t1054: F, t1926: F, t995: F, t1919: F, t210: F, t1933: F, t40: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6688 = t976 * t344;
    let t6689 = t6688 * t381;
    let t6690 = t225 * t387;
    let t6703 = t340 * t344;
    let t6704 = t6703 * t381;
    let t6705 = t225 * t1054;
    let t6716 = t1926 * t995 / F::new(288.0);
    let t6717 = t1919 * t210;
    let t6726 = t1933 * t40;
    (t6688, t6689, t6690, t6703, t6704, t6705, t6716, t6717, t6726)
}
