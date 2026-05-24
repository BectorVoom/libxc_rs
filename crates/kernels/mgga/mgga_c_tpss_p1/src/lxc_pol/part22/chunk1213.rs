//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1213/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1213<F: Float>(t5: F, t18678: F, t117: F, t1844: F, t507: F, t18539: F, t3205: F) -> (F, F, F, F) {
    let t7 = piecewise3::<F>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t18679 = piecewise3::<F>(t8, F::new(0.0), t18678);
    let t18680 = t18679 * t117;
    let t18686 = t507 * t1844;
    let t18687 = t18686 * t18539;
    let t18690 = t1844 * t3205;
    (t18679, t18680, t18687, t18690)
}
