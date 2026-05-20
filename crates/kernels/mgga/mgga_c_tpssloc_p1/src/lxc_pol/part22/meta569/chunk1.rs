//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2077/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2077<F: Float>(t221: F, t339: F, t42813: F, t10216: F, t2978: F, t10479: F, t42333: F, t3061: F, t676: F, t11065: F, t42387: F, t1005: F, t10375: F) -> (F, F, F, F, F, F) {
    let t43307 = F::new(5.0) / F::new(486.0) * t339 * t221 * t42813;
    let t43317 = t2978 * t10216;
    let t43322 = t42333 * t10479;
    let t43338 = t676 * t3061;
    let t43361 = t11065 * t42387;
    let t43382 = t1005 * t10375;
    (t43307, t43317, t43322, t43338, t43361, t43382)
}
