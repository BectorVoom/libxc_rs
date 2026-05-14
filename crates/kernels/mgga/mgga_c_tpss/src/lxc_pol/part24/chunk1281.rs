//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1281/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1281<F: Float>(t1679: F, t4570: F, t1678: F, t42667: F, t5486: F, t13298: F, t582: F, t10292: F, t19424: F, t1680: F, t19346: F, t19349: F, t19388: F, t19393: F, t19396: F, t5489: F, t6087: F, t62019: F, t65158: F, t65166: F, t65169: F, t65172: F, t65175: F, t65198: F) -> (F,) {
    let t69152 = t1679 * t4570;
    let t69153 = t1678 * t69152;
    let t69162 = t42667 * t5486;
    let t69165 = t13298 * t582;
    let t69168 = t10292 * t19424;
    let t69181 = 10.0 * t62019 * t69153 - 10.0 / 3.0 * t65169 * t19346 - 10.0 / 3.0 * t65172 * t19346 - 10.0 / 3.0 * t65175 * t19346 + 5.0 / 6.0 * t69162 * t5489 + t69165 * t1680 / 3.0 + 5.0 / 3.0 * t69168 * t5489 + 2.0 / 3.0 * t19396 * t6087 + 5.0 / 3.0 * t19393 * t19388 - 10.0 / 3.0 * t19349 * t65198 - 10.0 / 3.0 * t19349 * t65158 - 10.0 / 3.0 * t19349 * t65166;
    (t69181,)
}
