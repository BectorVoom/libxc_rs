//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1286/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1286<F: Float>(t10287: F, t577: F, t1980: F, t3416: F, t1286: F, t7689: F, t1321: F, t2105: F, t3490: F, t645: F, t1268: F, t4397: F) -> (F, F, F, F, F, F) {
    let t41937 = t10287 * t577;
    let t42178 = t3416 * t1980;
    let t42181 = t1286 * t7689;
    let t42336 = t1321 * t2105;
    let t42719 = t3490 * t645;
    let t42962 = t4397 * t1268;
    (t41937, t42178, t42181, t42336, t42719, t42962)
}
