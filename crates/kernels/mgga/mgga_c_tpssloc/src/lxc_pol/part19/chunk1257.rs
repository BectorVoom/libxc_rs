//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1257/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1257<F: Float>(t1070: F, t11094: F, t193: F, t3209: F, t3213: F, t336: F, t41804: F, t41813: F, t42276: F, t42280: F, t42283: F, t42663: F, t42665: F, t42667: F, t42669: F, t42674: F, t42678: F, t43447: F, t43622: F, t4700: F) -> (F,) {
    let t43627 = -t42276 - t42280 - t42283 + 12.0 * t4700 * t3213 * t11094 * t3209 + t193 * t336 * (t43447 + t43622) * t1070 + t42663 - t42665 + t41804 - t42667 + t42669 - t42674 - t41813 + t42678;
    (t43627,)
}
