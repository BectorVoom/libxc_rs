//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2444/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2444<F: Float>(t10224: F, t2995: F, t973: F, t10225: F, t2960: F, t10213: F, t135: F, t41961: F, t697: F, t976: F, t984: F, t2986: F, t2990: F) -> (F, F, F, F, F, F, F) {
    let t42962 = t973 * t10224 * t2995;
    let t42968 = t2960 * t10225;
    let t42972 = t135 * t10213;
    let t43002 = F::new(220.0) / F::new(81.0) * t41961;
    let t43052 = t697 * t976;
    let t43053 = t43052 * t984;
    let t43055 = t2986 * t43053 * t2990;
    (t42962, t42968, t42972, t43002, t43052, t43053, t43055)
}
