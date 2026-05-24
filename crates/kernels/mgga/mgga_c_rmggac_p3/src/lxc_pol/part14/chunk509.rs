//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 509/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk509<F: Float>(t495: F, t5542: F, t1173: F, t615: F, t1525: F, t461: F, t1175: F, t1240: F, t1510: F, t4559: F, t1182: F, t589: F) -> (F, F, F, F, F, F, F, F) {
    let t5543 = t5542 * t495;
    let t5554 = t1173 * t615;
    let t5555 = t5554 * t495;
    let t5558 = t461 * t1525;
    let t5561 = t615 * t1175;
    let t5564 = t1525 * t495;
    let t5567 = t615 * t1240;
    let t5571 = F::cast_from(0.25610252642437845428e0_f64) * t4559 * t1510;
    let t5572 = t589 * t1182;
    (t5543, t5555, t5558, t5561, t5564, t5567, t5571, t5572)
}
