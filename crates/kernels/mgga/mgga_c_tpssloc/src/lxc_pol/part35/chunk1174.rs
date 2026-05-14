//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1174/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1174<F: Float>(t23508: F, t8026: F, t7325: F, t27628: F, t7324: F, t15730: F, t7339: F, t3540: F, t8043: F, t27634: F, t1409: F, t461: F, t1009: F, t3545: F, t8020: F, t15753: F, t7310: F) -> (F, F, F, F, F, F, F, F) {
    let t95326 = t8026 * t23508;
    let t95327 = t95326 * t7325;
    let t95332 = t7324 * t27628;
    let t95335 = t7339 * t15730;
    let t95365 = t8043 * t3540;
    let t95387 = t27634 * t27628;
    let t95420 = t1409 * t461;
    let t95422 = t7324 * t95420 * t1009;
    let t95450 = t8020 * t3545;
    let t95512 = t7310 * t15753;
    (t95327, t95332, t95335, t95365, t95387, t95422, t95450, t95512)
}
