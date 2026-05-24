//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 1083/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk1083<F: Float>(t9042: F, t9052: F, t9058: F, t9065: F, t9077: F, t10061: F, t10062: F, t7891: F, t7893: F, t7896: F, t7898: F, t9612: F, t9613: F, t9614: F) -> (F, F, F, F) {
    let t42307 = F::cast_from(0.85129199786595678796e-5_f64) * t9042;
    let t42308 = F::cast_from(0.85129199786595678796e-5_f64) * t9052;
    let t42310 = F::cast_from(0.11974241701863808564e0_f64) * t9058;
    let t42312 = F::new(2.0) * t9065;
    let t42313 = F::cast_from(0.39914139006212695214e-1_f64) * t9077;
    let t42314 = t9612 + t42312 - t10061 + t7891 + t7893 - t9613 - t9614 + t10062 - t7896 + t7898 - t42313;
    (t42307, t42308, t42310, t42314)
}
