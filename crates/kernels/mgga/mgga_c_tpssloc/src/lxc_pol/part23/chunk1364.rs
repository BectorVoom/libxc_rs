//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1364/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1364<F: Float>(t39411: F, t39463: F, t39468: F, t39472: F, t39476: F, t39483: F, t79904: F, t79905: F, t79906: F, t79907: F, t79908: F, t79909: F, t79910: F, t39490: F, t39496: F, t39499: F, t39502: F, t39505: F, t39508: F, t39518: F, t39521: F, t39529: F, t39539: F, t39549: F, t79914: F) -> (F, F) {
    let t80105 = t39411 - t79904 - t79905 + t39463 - t39468 + t79906 - t39472 - t39476 - t79907 - t79908 - t79909 + t79910 + t39483;
    let t80108 = -t79914 - t39490 - t39496 + t39499 + t39502 - t39505 - t39508 + t39518 - t39521 - t39529 + t39539 + t39549;
    (t80105, t80108)
}
