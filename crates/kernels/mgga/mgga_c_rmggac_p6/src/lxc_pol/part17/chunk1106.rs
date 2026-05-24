//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1106/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1106<F: Float>(t10037: F, t10042: F, t10047: F, t10052: F, t10055: F, t34649: F, t7771: F, t9490: F, t9491: F, t9492: F, t9493: F, t10383: F, t10384: F, t42296: F, t42297: F, t42298: F, t42299: F, t42300: F, t42301: F, t42306: F, t9600: F, t9601: F) -> (F, F) {
    let t48064 = t34649 - t9490 + t9491 + t9492 - t9493 - t7771 - t10037 + t10042 + t10047 + t10052 - t10055;
    let t48067 = t42296 - t42297 + t42298 + t9600 + t9601 + t42299 + t42300 - t42301 + t42306 + t10383 - t10384;
    (t48064, t48067)
}
