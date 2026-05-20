//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1162/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1162<F: Float>(t10216: F, t2978: F, t3061: F, t676: F, t11065: F, t42387: F, t10475: F, t42342: F, t42345: F, t2770: F, t283: F, t61: F) -> (F, F, F, F, F) {
    let t43317 = t2978 * t10216;
    let t43338 = t676 * t3061;
    let t43361 = t11065 * t42387;
    let t43385 = t42342 * t10475 * t42345;
    let t43398 = F::new(1.0) / t283 / t2770;
    let t43399 = t61 * t43398;
    (t43317, t43338, t43361, t43385, t43399)
}
