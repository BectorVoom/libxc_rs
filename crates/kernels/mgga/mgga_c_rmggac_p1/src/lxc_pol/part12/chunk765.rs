//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 765/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk765<F: Float>(t31: F, t35604: F, t7349: F, t7351: F, t2019: F, t2020: F, t7220: F, t7224: F, t7338: F, t7345: F, t7341: F, t4905: F, t7778: F, t903: F) -> (F, F, F, F, F, F) {
    let t35728 = t7349 * t7351 * t35604 * t31;
    let t35729 = F::cast_from(0.65053455985619242968e-4_f64) * t35728;
    let t35731 = t2019 * t2020 * t7220;
    let t35737 = t2019 * t2020 * t7224;
    let t35742 = t7345 * t7338;
    let t35744 = t7345 * t7341;
    let t35752 = t903 * t7778 * t4905;
    (t35729, t35731, t35737, t35742, t35744, t35752)
}
