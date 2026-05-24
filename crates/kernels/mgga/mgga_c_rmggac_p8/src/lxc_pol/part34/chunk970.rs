//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 970/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk970<F: Float>(t71021: F, t68742: F, t3219: F, t38351: F, t38355: F, t14639: F, t8571: F, t15457: F, t16156: F, t74650: F, t74652: F, t74657: F) -> (F, F, F, F, F, F, F, F, F) {
    let t77121 = F::cast_from(0.21684485328539747656e-4_f64) * t71021;
    let t77123 = F::cast_from(0.79828278012425390427e-1_f64) * t68742;
    let t77124 = t38351 * t3219;
    let t77125 = F::cast_from(0.42564599893297839398e-5_f64) * t77124;
    let t77126 = t38355 * t3219;
    let t77127 = F::cast_from(0.42564599893297839398e-5_f64) * t77126;
    let t77128 = t8571 * t14639;
    let t77129 = F::cast_from(0.42564599893297839398e-5_f64) * t77128;
    let t77131 = t16156 * t15457;
    let t77132 = F::cast_from(0.29795219925308487578e-4_f64) * t77131;
    let t77134 = F::cast_from(0.2627895913935205078e-5_f64) * t74650;
    let t77135 = F::cast_from(0.12263514265030957031e-4_f64) * t74652;
    let t77137 = F::cast_from(0.54549323308490683456e-1_f64) * t74657;
    (t77121, t77123, t77125, t77127, t77129, t77132, t77134, t77135, t77137)
}
